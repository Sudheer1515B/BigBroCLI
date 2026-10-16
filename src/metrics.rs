use anyhow::{Context, Result, bail};

#[derive(Debug, Clone)]
pub struct Cputimez {
    pub name: String,
    pub total: u64,
    pub idle: u64,
}

#[derive(Debug)]
pub struct Metrics {
    pub overall: Cputimez,
    pub cores: Vec<Cputimez>,
    pub mem_total: u64,
    pub mem_available: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

fn parsecpu(text: &str) -> Result<Cputimez> {
    let parts: Vec<&str> = text.split(':').collect();

    if parts.len() != 3 {
        bail!("Invalid cpu data: {text}");
    }

    Ok(Cputimez {
        name: parts[0].to_string(),
        total: parts[1].parse(),
        idle: parts[2].parse(),
    })
}

pub fn parse(line: &str) -> Result<Metrics> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 6 {
        bail!("Incomplete metrics man, do somethin");
    }

    let overall = parse_cpu(parts[4])?;

    if overall.name != "cpu" {
        bail!("damn bruv where's yo overall CPU counters");
    }

    let cores = parts[5..]
        .iter()
        .map(|value| parse_cpu(value))
        .collect::<Result<Vec<_>>>()?;

    for core in &cores {
        let id = core.name.strip_prefix("cpu").context("Inval CPU name");

        if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
            bail!("Invalid core name, on {}", core.name);
        }
    }
    Ok(Metrics {
        overall,
        cores,

        mem_total: parts[0].parse(),
        mem_available: parts[1].parse(),
        swap_total: parts[2].parse(),
        swap_free: parts[3].parse(),
    })
}

pub fn cpu_usage(old: &Cputimez, new: &Cputimez) -> Option<f64> {
    if old.name != new.name {
        return None;
    }

    let total = new.total.checked_sub(old.total);
    let idle = new.idle.checked_sub(old.idle);

    if total == 0 || idle > total {
        return None;
    }

    Some(100.0 * (total - idle) as f64 / total as f64)
}
