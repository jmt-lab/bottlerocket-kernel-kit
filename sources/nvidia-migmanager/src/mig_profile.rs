use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum NvidiaA100_40gbMigProfile {
    #[serde(alias = "1g.5gb")]
    #[serde(alias = "7")]
    Mig1g5gb,

    #[serde(alias = "2g.10gb")]
    #[serde(alias = "3")]
    Mig2g10gb,

    #[serde(alias = "3g.20gb")]
    #[serde(alias = "2")]
    Mig3g20gb,

    #[serde(alias = "7g.40gb")]
    #[serde(alias = "1")]
    #[serde(other)]
    Mig7g40gb,
}

impl MigGpuProfile for NvidiaA100_40gbMigProfile {
    fn get_mig_profile(&self) -> &str {
        match self {
            NvidiaA100_40gbMigProfile::Mig7g40gb => "7g.40gb",
            NvidiaA100_40gbMigProfile::Mig3g20gb => "3g.20gb,3g.20gb",
            NvidiaA100_40gbMigProfile::Mig2g10gb => "2g.10gb,2g.10gb,2g.10gb",
            NvidiaA100_40gbMigProfile::Mig1g5gb => {
                "1g.5gb,1g.5gb,1g.5gb,1g.5gb,1g.5gb,1g.5gb,1g.5gb"
            }
        }
    }
}

#[derive(Deserialize)]
pub(crate) enum NvidiaA100_80gbMigProfile {
    #[serde(alias = "1g.10gb")]
    #[serde(alias = "7")]
    Mig1g10gb,

    #[serde(alias = "2g.20gb")]
    #[serde(alias = "3")]
    Mig2g20gb,

    #[serde(alias = "3g.40gb")]
    #[serde(alias = "2")]
    Mig3g40gb,

    #[serde(alias = "7g.80gb")]
    #[serde(alias = "1")]
    #[serde(other)]
    Mig7g80gb,
}

impl MigGpuProfile for NvidiaA100_80gbMigProfile {
    fn get_mig_profile(&self) -> &str {
        match self {
            NvidiaA100_80gbMigProfile::Mig7g80gb => "7g.80gb",
            NvidiaA100_80gbMigProfile::Mig3g40gb => "3g.40gb,3g.40gb",
            NvidiaA100_80gbMigProfile::Mig2g20gb => "2g.20gb,2g.20gb,2g.20gb",
            NvidiaA100_80gbMigProfile::Mig1g10gb => {
                "1g.10gb,1g.10gb,1g.10gb,1g.10gb,1g.10gb,1g.10gb,1g.10gb"
            }
        }
    }
}

#[derive(Deserialize)]
pub(crate) enum NvidiaH100_80gbMigProfile {
    #[serde(alias = "1g.10gb")]
    #[serde(alias = "7")]
    Mig1g10gb,

    #[serde(alias = "1g.20gb")]
    #[serde(alias = "4")]
    Mig1g20gb,

    #[serde(alias = "2g.20gb")]
    #[serde(alias = "3")]
    Mig2g20gb,

    #[serde(alias = "3g.40gb")]
    #[serde(alias = "2")]
    Mig3g40gb,

    #[serde(alias = "7g.80gb")]
    #[serde(alias = "1")]
    #[serde(other)]
    Mig7g80gb,
}

impl MigGpuProfile for NvidiaH100_80gbMigProfile {
    fn get_mig_profile(&self) -> &str {
        match self {
            NvidiaH100_80gbMigProfile::Mig7g80gb => "7g.80gb",
            NvidiaH100_80gbMigProfile::Mig3g40gb => "3g.40gb,3g.40gb",
            NvidiaH100_80gbMigProfile::Mig2g20gb => "2g.20gb,2g.20gb,2g.20gb",
            NvidiaH100_80gbMigProfile::Mig1g20gb => "1g.20gb,1g.20gb,1g.20gb,1g.20gb",
            NvidiaH100_80gbMigProfile::Mig1g10gb => {
                "1g.10gb,1g.10gb,1g.10gb,1g.10gb,1g.10gb,1g.10gb,1g.10gb"
            }
        }
    }
}

#[derive(Deserialize)]
pub(crate) enum NvidiaH200_141gbMigProfile {
    #[serde(alias = "1g.18gb")]
    #[serde(alias = "7")]
    Mig1g18gb,

    #[serde(alias = "1g.35gb")]
    #[serde(alias = "4")]
    Mig1g35gb,

    #[serde(alias = "2g.35gb")]
    #[serde(alias = "3")]
    Mig2g35gb,

    #[serde(alias = "3g.71gb")]
    #[serde(alias = "2")]
    Mig3g71gb,

    #[serde(alias = "7g.141gb")]
    #[serde(alias = "1")]
    #[serde(other)]
    Mig7g141gb,
}

impl MigGpuProfile for NvidiaH200_141gbMigProfile {
    fn get_mig_profile(&self) -> &str {
        match self {
            NvidiaH200_141gbMigProfile::Mig7g141gb => "7g.141gb",
            NvidiaH200_141gbMigProfile::Mig3g71gb => "3g.71gb,3g.71gb",
            NvidiaH200_141gbMigProfile::Mig2g35gb => "2g.35gb,2g.35gb,2g.35gb",
            NvidiaH200_141gbMigProfile::Mig1g35gb => "1g.35gb,1g.35gb,1g.35gb,1g.35gb",
            NvidiaH200_141gbMigProfile::Mig1g18gb => {
                "1g.18gb,1g.18gb,1g.18gb,1g.18gb,1g.18gb,1g.18gb,1g.18gb"
            }
        }
    }
}

pub(crate) trait MigGpuProfile: for<'de> Deserialize<'de> {
    fn get_mig_profile(&self) -> &str;
}
