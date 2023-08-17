use crate::error::Error;
use crate::raur::*;
use crate::{ArcPackage, Cache, AUR_URL};
use reqwest::blocking::Client;
use reqwest::Url;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "type")]
    response_type: String,
    error: Option<String>,
    results: Vec<Package>,
}

/// The main trait for RPC functionality.
///
/// This trait is implemented by [`Handle`], which is what you should use at run-time.
/// You can also use the mock implementation of this trait in e.g. tests.
pub trait Raur {
    /// Performs an AUR info request.
    ///
    /// You probably want to use [`info`] instead.
    ///
    /// This function sends an info request to the AUR RPC. This kind of request takes a list
    /// of package names and returns a list of AUR [`Package`](struct.Package.html)s who's name exactly matches
    /// the input.
    ///
    /// **Note:** If a package being queried does not exist then it will be silently ignored
    /// and not appear in return value.
    ///
    /// **Note:** The return value has no guaranteed order.
    fn raw_info<S: AsRef<str>>(&self, pkg_names: &[S]) -> Result<Vec<Package>, Error>;

    /// Performs an AUR info request, splitting requests as needed.
    ///
    /// This function sends an info request to the AUR RPC. This kind of request takes a list
    /// of package names and returns a list of AUR [`Package`](struct.Package.html)s who's name exactly matches
    /// the input.
    ///
    /// **Note:** If a package being queried does not exist then it will be silently ignored
    /// and not appear in return value.
    ///
    /// **Note:** The return value has no guaranteed order.
    fn info<S: AsRef<str>>(&self, pkg_names: &[S]) -> Result<Vec<Package>, Error> {
        let mut packages = Vec::with_capacity(pkg_names.len());

        for chunk in pkg_names.chunks(100) {
            packages.extend(self.raw_info(chunk)?);
        }

        Ok(packages)
    }

    /// Perform an info request, storing the results into cache. Requests are not made
    /// for packages already in cache. If all packages are already in cache then no network request
    /// will be made.
    ///
    /// The packages requested will be returned back (even if they were already in cache). Packages
    /// that could not be found will be missing from the return.
    fn cache_info<S: AsRef<str>>(
        &self,
        cache: &mut Cache,
        pkgs: &[S],
    ) -> Result<Vec<ArcPackage>, Error> {
        let mut ret = Vec::with_capacity(pkgs.len());
        let mut resolve = Vec::with_capacity(pkgs.len());

        for pkg in pkgs {
            if let Some(pkg) = cache.get(pkg.as_ref()) {
                ret.push(pkg.clone());
            } else {
                resolve.push(pkg.as_ref());
            }
        }

        cache.reserve(resolve.len());

        for chunk in resolve.chunks(100) {
            let res = self.info(chunk)?;
            for pkg in res.into_iter() {
                let pkg = ArcPackage::from(pkg);
                cache.insert(pkg.clone());
                ret.push(pkg);
            }
        }

        Ok(ret)
    }

    /// Performs an AUR search request.
    ///
    /// This function sends a search request to the AUR RPC. This kind of request takes a
    /// single query and returns a list of matching packages.
    ///
    /// **Note:** Unlike info, search results will never include:
    ///
    /// - Dependency types
    /// - Licences
    /// - Groups
    ///
    /// See [`SearchBy`](enum.SearchBy.html) for how packages are matched.
    fn search_by<S: AsRef<str>>(&self, query: S, strategy: SearchBy)
        -> Result<Vec<Package>, Error>;

    /// Performs an AUR search request by NameDesc.
    ///
    /// This is the same as [`fn.search_by`](fn.search_by.html) except it always searches by
    /// NameDesc (the default for the AUR).
    fn search<S: AsRef<str>>(&self, query: S) -> Result<Vec<Package>, Error> {
        self.search_by(query, SearchBy::NameDesc)
    }

    /// Returns a list of all orphan packages in the AUR.
    fn orphans(&self) -> Result<Vec<Package>, Error> {
        self.search_by("", SearchBy::Maintainer)
    }
}

/// A handle for making AUR requests.
#[derive(Clone, Debug)]
pub struct Handle {
    client: Client,
    url: String,
}

impl Raur for Handle {
    fn raw_info<S: AsRef<str>>(&self, pkg_names: &[S]) -> Result<Vec<Package>, Error> {
        let mut params = pkg_names
            .iter()
            .map(|name| ("arg[]", name.as_ref()))
            .collect::<Vec<_>>();
        params.extend(&[("v", "5"), ("type", "info")]);

        self.request(&params)
    }

    fn search_by<S: AsRef<str>>(
        &self,
        query: S,
        strategy: SearchBy,
    ) -> Result<Vec<Package>, Error> {
        self.request(&[
            ("v", "5"),
            ("type", "search"),
            ("by", &strategy.to_string()),
            ("arg", query.as_ref()),
        ])
    }
}

impl Default for Handle {
    fn default() -> Self {
        Handle {
            client: Client::new(),
            url: AUR_URL.to_string(),
        }
    }
}

impl Handle {
    /// Create a new handle with default settings.
    pub fn new() -> Self {
        Handle {
            client: Client::new(),
            url: AUR_URL.to_string(),
        }
    }

    /// Create a new handle with a given reqwest client.
    pub fn new_with_client(client: Client) -> Self {
        Handle {
            client,
            url: AUR_URL.to_string(),
        }
    }

    /// Create a new handle with a given url.
    pub fn new_with_url<S: Into<String>>(url: S) -> Self {
        Handle {
            client: Client::new(),
            url: url.into(),
        }
    }

    /// Create a new handle with a given reqwest client and url.
    pub fn new_with_settings<S: Into<String>>(client: Client, url: S) -> Self {
        Handle {
            client,
            url: url.into(),
        }
    }

    /// Internal URL of this Handle. This just points to AUR_URL if you did not explicitly
    /// set it.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Getter for this handle's reqwest client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// A helper function for making a request with given parameters.
    fn request(&self, params: &[(&str, &str)]) -> Result<Vec<Package>, Error> {
        let new_url = Url::parse_with_params(&self.url, params)?;

        let response = self.client.get(new_url).send()?;
        response.error_for_status_ref()?;
        let response: Response = response.json()?;

        if response.response_type == "error" {
            Err(Error::Aur(
                response
                    .error
                    .unwrap_or_else(|| "No error message provided".to_string()),
            ))
        } else {
            Ok(response.results)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let handle = Handle::default();

        let query = handle.search("zzzzzzz").unwrap();
        assert_eq!(0, query.len());

        let query = handle.search("spotify").unwrap();
        assert!(!query.is_empty());
        assert!(query[0].url.is_some());
    }

    #[test]
    fn test_info() {
        let handle = Handle::default();

        let query = handle.info(&["pacman-git"]).unwrap();
        assert_eq!(query[0].name, "pacman-git");

        // I maintain these two packages, so I can verify they exist.
        let query = handle.info(&["screens", "screens-git"]);
        assert!(query.is_ok());

        let query = query.unwrap();
        assert_eq!(2, query.len());
    }
}
