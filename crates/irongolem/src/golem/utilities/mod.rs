//! Job-count helper matching Python `golem.utilities.utilities.determine_n_jobs`.

pub fn determine_n_jobs(n_jobs: i32) -> Result<usize, String> {
    let cpu_num = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    if n_jobs > cpu_num as i32 {
        return Ok(cpu_num);
    }
    if n_jobs <= 0 {
        if n_jobs < -(cpu_num as i32) || n_jobs == 0 {
            return Err(format!("Unproper `n_jobs` = {n_jobs}"));
        }
        return Ok((cpu_num as i32 + 1 + n_jobs) as usize);
    }
    Ok(n_jobs as usize)
}
