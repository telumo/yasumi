# 🎌 Yasumi - Blazing Fast Japanese Holidays for Python

**Yasumi** is a high-performance Python library for determining Japanese holidays, built with Rust and powered by [Maturin](https://github.com/PyO3/maturin). Yasumi offers the same functionality as the popular [jpholiday](https://github.com/Lalcs/jpholiday) library, but with **65x faster performance** thanks to Rust's speed and efficiency.

If you're working with Japanese holidays in Python and need a significant boost in performance, Yasumi is your best choice!

## 🚀 Features

- 🌸 **65x faster** than `jpholiday` for holiday calculations.
- 🎏 Determine if a date is a Japanese holiday.
- 📅 Retrieve the name of the holiday on a specific date.
- 📆 Fetch holiday lists for any given year or range of dates.

## 🔧 Installation

You can easily install Yasumi via `pip`:

```bash
pip install yasumi
```

## 📖 Usage

Here’s how you can use Yasumi in your Python projects:

```python
import yasumi
import datetime

# Check if a specific date is a holiday
date = datetime.date(2024, 1, 1) # or datetime.datetime is also available.
if yasumi.is_holiday(date):
    print(f"{date} is a holiday: {yasumi.holiday_name(date)}")
else:
    print(f"{date} is not a holiday.")

# Get all holidays for a given year
holidays = yasumi.year_holidays(2024)
for date, name in holidays:
    print(f"Holiday on {date}: {name}")
```

### Available Functions

- is_holiday_name(date: datetime | date) -> str | None
Check if the given date is a holiday and get its name, if available.
- holiday_name(date: datetime | date) -> str | None
Get the name of the holiday on the given date, if it’s a holiday.
- is_holiday(date: datetime | date) -> bool
Check if the given date is a holiday.
- is_no_workday(date: datetime | date) -> bool
Determine if the given date is a non-working day (including holidays).
- month_holidays(year: int, month: int) -> list[tuple[date, str]]
Get a list of holidays for a specific month in a given year.
- year_holidays(year: int) -> list[tuple[date, str]]
Get a list of all holidays in a given year.
- holidays(start_date: datetime | date, end_date: datetime | date) -> list[tuple[date, str]]
Get a list of holidays between the specified start and end dates.
- between(start_date: datetime | date, end_date: datetime | date) -> list[tuple[date, str]]
Retrieve holidays that fall between the specified dates.

## 💡 Why Yasumi?

Yasumi is built with Rust, making it 65x faster than the Python-based jpholiday. This makes it perfect for applications where performance is critical, such as web services, data processing, and large-scale holiday calculations. With Yasumi, you get the speed of Rust with the ease of Python.

## 🛠 Development

If you want to build Yasumi from source, clone the repository and use Maturin to build the package:

```bash
git clone https://github.com/telumo/yasumi.git
cd yasumi
maturin build -i python3 --release
pip install .
```

To run tests:

```python
pytest
```

## 🎉 Credits

Yasumi is inspired by the [jpholiday](https://github.com/Lalcs/jpholiday) Python package. Special thanks to the jpholiday community and the contributors to Rust and Maturin that made this project possible.