use anyhow::anyhow;
use extendr_api::{error::Result, prelude::*};
use url::Url;

// this is for the shape examples
mod shape;

// The newtype that holds a Rust Url as an R external pointer
#[derive(Clone)]
#[extendr]
struct RUrl(Url);

#[extendr]
impl RUrl {
    // helper to parse the Url
    fn new(url: &str) -> anyhow::Result<Self> {
        Ok(Self(Url::parse(url)?))
    }

    // we use an external pointer when we want to get a value
    // from the object itself and not modify it
    fn get_extptr(robj: Robj) -> Result<ExternalPtr<Self>> {
        ExternalPtr::<RUrl>::try_from(robj)
    }

    // Perform a deep clone from the external pointer
    // we want to clone in the events that we will change
    // the values in the url via the setters
    fn clone_from_robj(robj: Robj) -> anyhow::Result<Self> {
        match ExternalPtr::<RUrl>::try_from(robj) {
            Ok(v) => Ok(v.as_ref().clone()),
            Err(e) => Err(anyhow!("Expected an object of class `rurl`:\n{e}")),
        }
    }
}

#[extendr]
fn url_parse(url: Strings) -> Result<List> {
    let mut res = url
        .into_iter()
        .map(|xi| {
            if xi.is_na() {
                return ().into_robj();
            }

            match RUrl::new(xi) {
                Ok(v) => v.into_robj(),
                Err(_) => ().into_robj(),
            }
        })
        .collect::<List>();

    res.set_class(&["rurls", "vctrs_vctr", "list"])?;
    Ok(res)
}

#[extendr]
fn format_rurls(x: List) -> Strings {
    // when iterating over a list the value is a tuple
    // the first element i the name and the second is the
    // value as an Robj
    x.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.to_string()),
                Err(_) => Rstr::na(),
            }
        })
        .collect::<Strings>()
}

#[extendr]
fn url_host(url: List) -> Strings {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.host_str().map(|v| v.to_string())),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_scheme(url: List) -> Strings {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.scheme()),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_path(url: List) -> Strings {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.path()),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_query(url: List) -> Strings {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.query().map(|v| v.to_string())),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_authority(url: List) -> Strings {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.authority()),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_domain(url: List) -> Strings {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                Ok(rurl) => Rstr::from(rurl.0.domain().map(|vi| vi.to_string())),
                Err(_) => Rstr::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_port(url: List) -> Integers {
    url.into_iter()
        .map(|(_, vi)| {
            let vv = RUrl::get_extptr(vi);
            match vv {
                // we have to cast from an unsigned integer to a signed 32 bit one
                // this is to match how R represents integers
                Ok(rurl) => Rint::from(rurl.0.port().map(|inner| inner as i32)),
                Err(_) => Rint::na(),
            }
        })
        .collect()
}

#[extendr]
fn url_join(url: List, segment: Strings) -> Result<List> {
    if url.len() != segment.len() {
        throw_r_error("`url` and `segment` must be the same length");
    }

    let mut res = url
        .into_iter()
        .zip(segment.into_iter())
        .map(|((_, url), segment)| {
            if url.is_null() | segment.is_na() {
                return Robj::from(());
            }

            match RUrl::clone_from_robj(url) {
                Ok(url) => match url.0.join(segment) {
                    Ok(res) => RUrl(res).into(),
                    Err(_) => ().into(),
                },
                Err(_) => ().into(),
            }
        })
        .collect::<List>();

    res.set_class(&["rurls", "vctrs_vctr", "list"])?;
    Ok(res)
}

// this should be done at the end since its super tough
// returns a list of character vectors
#[extendr]
fn url_path_segments(url: List) -> List {
    url.into_iter()
        .map(|(_, robj)| match RUrl::get_extptr(robj) {
            Ok(v) => match v.0.path_segments() {
                // each segment is a `&str`; collect them into a character vector
                Some(segments) => segments
                    .map(|vi| Rstr::from(vi))
                    .collect::<Strings>()
                    .into_robj(),
                // if none, there are no segments
                None => ().into_robj(),
            },
            Err(_) => ().into_robj(),
        })
        .collect()
}

// ---- setters: copy semantics, return new list of RUrl ----
// Each setter clones the url out of the external pointer (so we never mutate the
// caller's object in place), applies the change, and returns a brand new list of
// `RUrl`s. The setter value is recycled over the urls with `.cycle()`, so a
// length-1 value applies to every url—just like R's recycling rules.

#[extendr]
fn url_set_host(url: List, host: Strings) -> List {
    if (host.len() != url.len()) & (host.len() != 1) {
        throw_r_error("`host` must be a scalar or the same length as `url`");
    }
    url.into_iter()
        .zip(host.into_iter().cycle())
        .map(|((_, url), host)| {
            if url.is_null() || host.is_na() {
                return ().into_robj();
            }

            match RUrl::clone_from_robj(url) {
                Ok(mut rurl) => match rurl.0.set_host(Some(host)) {
                    Ok(_) => rurl.into_robj(),
                    Err(_) => ().into_robj(),
                },
                Err(_) => ().into_robj(),
            }
        })
        .collect()
}

#[extendr]
fn url_set_path(url: List, path: Strings) -> List {
    if (path.len() != url.len()) & (path.len() != 1) {
        throw_r_error("`path` must be a scalar or the same length as `url`");
    }
    url.into_iter()
        .zip(path.into_iter().cycle())
        .map(|((_, url), path)| {
            if url.is_null() || path.is_na() {
                return ().into_robj();
            }

            match RUrl::clone_from_robj(url) {
                Ok(mut rurl) => {
                    // `set_path` is infallible—it always succeeds
                    rurl.0.set_path(path);
                    rurl.into_robj()
                }
                Err(_) => ().into_robj(),
            }
        })
        .collect()
}

#[extendr]
fn url_set_port(url: List, port: Integers) -> List {
    if (port.len() != url.len()) & (port.len() != 1) {
        throw_r_error("`port` must be a scalar or the same length as `url`");
    }
    url.into_iter()
        .zip(port.into_iter().cycle())
        .map(|((_, url), port)| {
            if url.is_null() || port.is_na() {
                return ().into_robj();
            }

            match RUrl::clone_from_robj(url) {
                // ports are unsigned 16-bit; cast from R's i32
                Ok(mut rurl) => match rurl.0.set_port(Some(port.0 as u16)) {
                    Ok(_) => rurl.into_robj(),
                    Err(_) => ().into_robj(),
                },
                Err(_) => ().into_robj(),
            }
        })
        .collect()
}

#[extendr]
fn url_set_scheme(url: List, scheme: Strings) -> List {
    if (scheme.len() != url.len()) & (scheme.len() != 1) {
        throw_r_error("`scheme` must be a scalar or the same length as `url`");
    }
    url.into_iter()
        .zip(scheme.into_iter().cycle())
        .map(|((_, url), scheme)| {
            if url.is_null() || scheme.is_na() {
                return ().into_robj();
            }

            match RUrl::clone_from_robj(url) {
                Ok(mut rurl) => match rurl.0.set_scheme(scheme) {
                    Ok(_) => rurl.into_robj(),
                    Err(_) => ().into_robj(),
                },
                Err(_) => ().into_robj(),
            }
        })
        .collect()
}

#[extendr]
fn url_query_pairs(url: List) -> List {
    url.into_iter()
        .map(|(_, robj)| match RUrl::get_extptr(robj) {
            Ok(rurl) => rurl
                .0
                .query_pairs()
                .map(|(key, value)| {
                    Strings::from_values([
                        Rstr::from(key.to_string()),
                        Rstr::from(value.to_string()),
                    ])
                    .into_robj()
                })
                .collect::<List>()
                .into_robj(),
            Err(_) => ().into_robj(),
        })
        .collect()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rurls;
    impl RUrl;
    fn url_parse;
    fn format_rurls;
    fn url_host;
    fn url_scheme;
    fn url_path;
    fn url_port;
    fn url_query;
    fn url_authority;
    fn url_domain;
    fn url_join;
    fn url_path_segments;
    fn url_set_host;
    fn url_set_path;
    fn url_set_port;
    fn url_set_scheme;
    fn url_query_pairs;
}
