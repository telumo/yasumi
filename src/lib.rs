use chrono::{Datelike, NaiveDate};
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateTime, PyList};

#[pymodule(name = "yasumi")]
fn py_yasumi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_is_holiday_name, m)?)?;
    m.add_function(wrap_pyfunction!(py_holiday_name, m)?)?;
    m.add_function(wrap_pyfunction!(py_is_holiday, m)?)?;
    m.add_function(wrap_pyfunction!(py_is_no_workday, m)?)?;
    m.add_function(wrap_pyfunction!(py_month_holidays, m)?)?;
    m.add_function(wrap_pyfunction!(py_year_holidays, m)?)?;
    m.add_function(wrap_pyfunction!(py_holidays, m)?)?;
    m.add_function(wrap_pyfunction!(py_between, m)?)?;
    Ok(())
}

#[pyfunction(name = "is_holiday_name")]
fn py_is_holiday_name(date: &Bound<'_, PyAny>) -> PyResult<Option<String>> {
    if date.is_instance_of::<PyDateTime>() || date.is_instance_of::<PyDate>() {
        let year = date.getattr("year")?.extract()?;
        let month = date.getattr("month")?.extract()?;
        let day = date.getattr("day")?.extract()?;
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid date provided"))?;
        return Ok(yasumi::is_holiday_name(date));
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a datetime.datetime or datetime.date object",
    ))
}

#[pyfunction(name = "holiday_name")]
fn py_holiday_name(date: &Bound<'_, PyAny>) -> PyResult<Option<String>> {
    if date.is_instance_of::<PyDateTime>() || date.is_instance_of::<PyDate>() {
        let year = date.getattr("year")?.extract()?;
        let month = date.getattr("month")?.extract()?;
        let day = date.getattr("day")?.extract()?;
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid date provided"))?;
        return Ok(yasumi::holiday_name(date));
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a datetime.datetime or datetime.date object",
    ))
}

#[pyfunction(name = "is_holiday")]
fn py_is_holiday(date: &Bound<'_, PyAny>) -> PyResult<bool> {
    if date.is_instance_of::<PyDateTime>() || date.is_instance_of::<PyDate>() {
        let year = date.getattr("year")?.extract()?;
        let month = date.getattr("month")?.extract()?;
        let day = date.getattr("day")?.extract()?;
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid date provided"))?;
        return Ok(yasumi::is_holiday(date));
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a datetime.datetime or datetime.date object",
    ))
}

#[pyfunction(name = "is_no_workday")]
fn py_is_no_workday(date: &Bound<'_, PyAny>) -> PyResult<bool> {
    if date.is_instance_of::<PyDateTime>() || date.is_instance_of::<PyDate>() {
        let year = date.getattr("year")?.extract()?;
        let month = date.getattr("month")?.extract()?;
        let day = date.getattr("day")?.extract()?;
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid date provided"))?;
        return Ok(yasumi::is_no_workday(date));
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a datetime.datetime or datetime.date object",
    ))
}

#[pyfunction(name = "month_holidays")]
fn py_month_holidays(py: Python<'_>, year: i32, month: u32) -> PyResult<Bound<'_, PyList>> {
    let holidays: Vec<(NaiveDate, String)> = yasumi::month_holidays(year, month);

    let py_holidays = PyList::empty_bound(py);

    for (date, name) in holidays {
        let py_date =
            PyDate::new_bound(py, date.year(), date.month() as u8, date.day() as u8).unwrap();
        let py_tuple = (py_date, name).to_object(py);
        py_holidays.append(py_tuple)?;
    }

    Ok(py_holidays)
}

#[pyfunction(name = "year_holidays")]
fn py_year_holidays(py: Python<'_>, year: i32) -> PyResult<Bound<'_, PyList>> {
    let holidays: Vec<(NaiveDate, String)> = yasumi::year_holidays(year);

    let py_holidays = PyList::empty_bound(py);

    for (date, name) in holidays {
        let py_date =
            PyDate::new_bound(py, date.year(), date.month() as u8, date.day() as u8).unwrap();
        let py_tuple = (py_date, name).to_object(py);
        py_holidays.append(py_tuple)?;
    }

    Ok(py_holidays)
}

#[pyfunction(name = "holidays")]
fn py_holidays<'py>(
    py: Python<'py>,
    start_date: &Bound<'_, PyAny>,
    end_date: &Bound<'_, PyAny>,
) -> PyResult<Bound<'py, PyList>> {
    if (start_date.is_instance_of::<PyDateTime>() || start_date.is_instance_of::<PyDate>())
        && (end_date.is_instance_of::<PyDateTime>() || end_date.is_instance_of::<PyDate>())
    {
        let start_year = start_date.getattr("year")?.extract()?;
        let start_month = start_date.getattr("month")?.extract()?;
        let start_day = start_date.getattr("day")?.extract()?;
        let start_date =
            NaiveDate::from_ymd_opt(start_year, start_month, start_day).ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err("Invalid start date provided")
            })?;

        let end_year = end_date.getattr("year")?.extract()?;
        let end_month = end_date.getattr("month")?.extract()?;
        let end_day = end_date.getattr("day")?.extract()?;
        let end_date = NaiveDate::from_ymd_opt(end_year, end_month, end_day)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid end date provided"))?;

        let holidays: Vec<(NaiveDate, String)> = yasumi::holidays(start_date, end_date);

        let py_holidays = PyList::empty_bound(py);

        for (date, name) in holidays {
            let py_date =
                PyDate::new_bound(py, date.year(), date.month() as u8, date.day() as u8).unwrap();
            let py_tuple = (py_date, name).to_object(py);
            py_holidays.append(py_tuple)?;
        }

        return Ok(py_holidays);
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a datetime.datetime or datetime.date object",
    ))
}

#[pyfunction(name = "between")]
fn py_between<'py>(
    py: Python<'py>,
    start_date: &Bound<'_, PyAny>,
    end_date: &Bound<'_, PyAny>,
) -> PyResult<Bound<'py, PyList>> {
    if (start_date.is_instance_of::<PyDateTime>() || start_date.is_instance_of::<PyDate>())
        && (end_date.is_instance_of::<PyDateTime>() || end_date.is_instance_of::<PyDate>())
    {
        let start_year = start_date.getattr("year")?.extract()?;
        let start_month = start_date.getattr("month")?.extract()?;
        let start_day = start_date.getattr("day")?.extract()?;
        let start_date =
            NaiveDate::from_ymd_opt(start_year, start_month, start_day).ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err("Invalid start date provided")
            })?;

        let end_year = end_date.getattr("year")?.extract()?;
        let end_month = end_date.getattr("month")?.extract()?;
        let end_day = end_date.getattr("day")?.extract()?;
        let end_date = NaiveDate::from_ymd_opt(end_year, end_month, end_day)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Invalid end date provided"))?;

        let holidays: Vec<(NaiveDate, String)> = yasumi::between(start_date, end_date);

        let py_holidays = PyList::empty_bound(py);

        for (date, name) in holidays {
            let py_date =
                PyDate::new_bound(py, date.year(), date.month() as u8, date.day() as u8).unwrap();
            let py_tuple = (py_date, name).to_object(py);
            py_holidays.append(py_tuple)?;
        }

        return Ok(py_holidays);
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected a datetime.datetime or datetime.date object",
    ))
}
