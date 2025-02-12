use std::sync::Arc;

use serde::{
    de::{self, value::StrDeserializer, IntoDeserializer},
    Deserialize, Deserializer, Serialize,
};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    #[serde(rename = "repo")]
    pub meta: Meta,
    #[serde(rename = "application")]
    pub apps: Option<Vec<App>>,
}

impl Default for Repo {
    fn default() -> Self {
        Self {
            meta: Meta {
                icon: "icon.png".into(),
                max_age: "14".into(),
                name: "F-Droid".into(),
                pub_key: "3082035e30820246a00302010202044c49cd00300d06092a864886f70d01010505003071310b300906035504061302554b3110300e06035504081307556e6b6e6f776e3111300f0603550407130857657468657262793110300e060355040a1307556e6b6e6f776e3110300e060355040b1307556e6b6e6f776e311930170603550403131043696172616e2047756c746e69656b73301e170d3130303732333137313032345a170d3337313230383137313032345a3071310b300906035504061302554b3110300e06035504081307556e6b6e6f776e3111300f0603550407130857657468657262793110300e060355040a1307556e6b6e6f776e3110300e060355040b1307556e6b6e6f776e311930170603550403131043696172616e2047756c746e69656b7330820122300d06092a864886f70d01010105000382010f003082010a028201010096d075e47c014e7822c89fd67f795d23203e2a8843f53ba4e6b1bf5f2fd0e225938267cfcae7fbf4fe596346afbaf4070fdb91f66fbcdf2348a3d92430502824f80517b156fab00809bdc8e631bfa9afd42d9045ab5fd6d28d9e140afc1300917b19b7c6c4df4a494cf1f7cb4a63c80d734265d735af9e4f09455f427aa65a53563f87b336ca2c19d244fcbba617ba0b19e56ed34afe0b253ab91e2fdb1271f1b9e3c3232027ed8862a112f0706e234cf236914b939bcf959821ecb2a6c18057e070de3428046d94b175e1d89bd795e535499a091f5bc65a79d539a8d43891ec504058acb28c08393b5718b57600a211e803f4a634e5c57f25b9b8c4422c6fd90203010001300d06092a864886f70d0101050500038201010008e4ef699e9807677ff56753da73efb2390d5ae2c17e4db691d5df7a7b60fc071ae509c5414be7d5da74df2811e83d3668c4a0b1abc84b9fa7d96b4cdf30bba68517ad2a93e233b042972ac0553a4801c9ebe07bf57ebe9a3b3d6d663965260e50f3b8f46db0531761e60340a2bddc3426098397fda54044a17e5244549f9869b460ca5e6e216b6f6a2db0580b480ca2afe6ec6b46eedacfa4aa45038809ece0c5978653d6c85f678e7f5a2156d1bedd8117751e64a4b0dcd140f3040b021821a8d93aed8d01ba36db6c82372211fed714d9a32607038cdfd565bd529ffc637212aaa2c224ef22b603eccefb5bf1e085c191d4b24fe742b17ab3f55d4e6f05ef".into(),
                timestamp: 0,
                url: Some("https://f-droid.org/repo/".into()),
                version: None,
                desc: None,
                mirrors: None
            },
            apps: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename = "@icon")]
    pub icon: String,
    #[serde(rename = "@maxage")]
    pub max_age: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@pubkey")]
    pub pub_key: String,
    #[serde(rename = "@timestamp")]
    pub timestamp: u64,
    #[serde(rename = "@url")]
    pub url: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<u32>,
    #[serde(rename = "description")]
    pub desc: Option<String>,
    #[serde(rename = "mirror")]
    pub mirrors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub id: String,
    pub added: String,
    #[serde(rename = "lastupdated")]
    pub last_updated: String,
    // 50 limit
    pub name: String,
    // 80 limit
    pub summary: String,
    pub icon: Option<String>,
    // limit 4000
    pub desc: String,
    pub license: String,
    #[serde(deserialize_with = "split_by_comma")]
    pub categories: Vec<Category>,
    pub category: Category,
    pub web: Option<String>,
    pub source: Option<String>,
    pub tracker: Option<String>,
    pub changelog: Option<String>,
    pub author: Option<String>,
    pub email: Option<String>,
    pub donate: Option<String>,
    pub bitcoin: Option<String>,
    #[serde(rename = "openCollective")]
    pub open_collective: Option<String>,
    #[serde(rename = "marketversion")]
    pub market_version: String,
    #[serde(rename = "marketvercode")]
    pub market_version_code: u32,
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub version: String,
    #[serde(rename = "versioncode")]
    pub version_code: u32,
    #[serde(rename = "apkname")]
    pub apk_name: String,
    #[serde(rename = "srcname")]
    pub src_name: Option<String>,
    pub hash: String,
    pub size: u32,
    #[serde(rename = "sdkver")]
    pub sdk_version: Option<u32>,
    #[serde(rename = "targetSdkVersion")]
    pub target_sdk_version: Option<u32>,
    pub added: String,
    pub sig: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub native_code: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
}

/// The [Category](https://f-droid.org/en/docs/Build_Metadata_Reference/#Categories) of the package.
/// Preferably a predefined category like [Category::Games] or [Category::Money], but can also
/// be a custom Category (see [Category::Custom]).
///
/// During Deserialization, if a categories does not match any category defined in the enum,
/// it automatically gets assigned to [Category::Custom]
#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    Connectivity,
    Development,
    Games,
    Graphics,
    Internet,
    Money,
    Multimedia,
    Navigation,
    #[serde(rename = "Phone & SMS")]
    PhoneSms,
    Reading,
    #[serde(rename = "Science & Education")]
    ScienceEducation,
    Security,
    #[serde(rename = "Sports & Health")]
    SportsHealth,
    System,
    Theming,
    Time,
    Writing,
    Custom(String),
}

/// The type of repository - for automatic building from source. If this is not specified, automatic building is disabled for this application.
///
/// See [documentation](https://f-droid.org/en/docs/Build_Metadata_Reference/#RepoType)
#[derive(Serialize, Deserialize, Debug)]
pub enum RepoType {
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "svn")]
    Svn,
    #[serde(rename = "git-svn")]
    GitSvn,
    #[serde(rename = "hg")]
    Hg,
    #[serde(rename = "bzr")]
    Bzr,
    #[serde(rename = "srclib")]
    Srclib,
}

/// Features of the application that hinders the user.
///
/// See [anti-feature](https://en.wiktionary.org/wiki/anti-feature)
#[derive(Serialize, Deserialize, Debug)]
pub enum AntiFeature {
    /// The application contains advertising
    Ads,
    /// User or activity data is tracked or leaks, by default. True if the app or a feature can not be used without collecting and sharing such data, or doing requests to a data collecting network service (regard- less if the service is based on free software, or not). For example, activity-based down-loading of weather data, maps, avatars etc. (data hosting and delivery services), or uploading of crash logs etc.
    Tracking,
    /// The application contains a feature that promotes or depends on a Non-Free network service which is impossible, or not easy to replace. Replacement requires changes to the app or service. This antifeature would not apply, if there is a simple configuration option that allows pointing the app to a running instance of an alternative, publicly available, self-hostable, free software server solution.
    NonFreeNet,
    /// The application promotes Non-Free add-ons, such that the app is effectively an advert for other Non-Free Software.
    NonFreeAdd,
    /// The application depends on a Non-Free application (e.g. Google Maps) - i.e. it requires it to be installed on the device, but does not include it.
    NonFreeDep,
    #[serde(rename = "NSFW")]
    /// The app contains content that the user may not want to be publicized or visible everywhere, comes from “Not Safe For Work”.
    Nsfw,
    /// The application is or depends on Non-Free software. This does not mean that Non-Free Software is included with the app: Most likely, it has been patched in some way to remove the Non-Free code. However, functionality may be missing.
    UpstreamNonFree,
    /// The application contains and makes use of Non-Free assets. The most common case is apps using artwork - images, sounds, music, etc. - under a license that restricts commercial usage or making derivative works (for example, any Creative Commons license with a “Non-Commercial” (NC) or “No Derivatives” (ND) restriction).
    NonFreeAssets,
    /// The application has known security vulnerabilities.
    KnownVuln,
    /// APK file is compiled for debugging (application-debuggable), which normally makes it unsuitable for regular users and use cases.
    ApplicationDebuggable,
    /// Upstream source for this app is no longer available. Either the app went commercial, the repo was dropped, or it has moved to a location currently unknown to us. This usually means there won’t be further updates unless the source reappears.
    NoSourceSince,
}

fn split_by_comma<'de, D>(deserializer: D) -> Result<Vec<Category>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.split(',')
        .map(|s| {
            let trimmed = s.trim();
            let deserializer: StrDeserializer<D::Error> = trimmed.into_deserializer();
            Category::deserialize(deserializer).map_err(de::Error::custom)
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn deserialize_mutex<'de, D>(deserializer: D) -> Result<Vec<Arc<Mutex<Repo>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let repos: Vec<Repo> = Vec::deserialize(deserializer)?;
    Ok(repos
        .into_iter()
        .map(|repo| Arc::new(Mutex::new(repo)))
        .collect())
}
