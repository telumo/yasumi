from datetime import datetime, date

import pytest
import yasumi
import jpholiday # type: ignore[import-untyped]

# NOTE: ロジックのテストは、Rust で実施している.
#       ここでは、Python のインターフェースのテストを行う.


@pytest.mark.parametrize(
    ("func", "date", "expected"),
    [
        # is_holiday_name
        (yasumi.is_holiday_name, datetime(2024, 1, 1), "元日"),
        (yasumi.is_holiday_name, datetime(2024, 1, 2), None),
        (yasumi.is_holiday_name, date(2024, 1, 1), "元日"),
        (yasumi.is_holiday_name, date(2024, 1, 2), None),
        # holiday_name
        (yasumi.holiday_name, datetime(2024, 1, 1), "元日"),
        (yasumi.holiday_name, datetime(2024, 1, 2), None),
        (yasumi.holiday_name, date(2024, 1, 1), "元日"),
        (yasumi.holiday_name, date(2024, 1, 2), None),
        # is_holiday
        (yasumi.is_holiday, datetime(2024, 1, 1), True),
        (yasumi.is_holiday, datetime(2024, 1, 2), False),
        (yasumi.is_holiday, date(2024, 1, 1), True),
        (yasumi.is_holiday, date(2024, 1, 2), False),
        # is_no_workday
        (yasumi.is_no_workday, datetime(2024, 1, 1), True),
        (yasumi.is_no_workday, datetime(2024, 1, 2), False),
        (yasumi.is_no_workday, date(2024, 1, 1), True),
        (yasumi.is_no_workday, date(2024, 1, 2), False),
    ],
)
def test_date(func, date, expected):
    func(date) == expected


def test_month_holidays():
    holidays = yasumi.month_holidays(2024, 1)
    assert holidays == [
        (date(2024, 1, 1), "元日"),
        (date(2024, 1, 8), "成人の日"),
    ]


def test_year_holidays():
    holidays = yasumi.year_holidays(2024)
    assert holidays == [
        (date(2024, 1, 1), "元日"),
        (date(2024, 1, 8), "成人の日"),
        (date(2024, 2, 11), "建国記念の日"),
        (date(2024, 2, 12), "建国記念の日 振替休日"),
        (date(2024, 2, 23), "天皇誕生日"),
        (date(2024, 3, 20), "春分の日"),
        (date(2024, 4, 29), "昭和の日"),
        (date(2024, 5, 3), "憲法記念日"),
        (date(2024, 5, 4), "みどりの日"),
        (date(2024, 5, 5), "こどもの日"),
        (date(2024, 5, 6), "こどもの日 振替休日"),
        (date(2024, 7, 15), "海の日"),
        (date(2024, 8, 11), "山の日"),
        (date(2024, 8, 12), "山の日 振替休日"),
        (date(2024, 9, 16), "敬老の日"),
        (date(2024, 9, 22), "秋分の日"),
        (date(2024, 9, 23), "秋分の日 振替休日"),
        (date(2024, 10, 14), "スポーツの日"),
        (date(2024, 11, 3), "文化の日"),
        (date(2024, 11, 4), "文化の日 振替休日"),
        (date(2024, 11, 23), "勤労感謝の日"),
    ]


def test_between():
    holidays = yasumi.between(date(2024, 1, 1), date(2024, 1, 8))
    assert holidays == [
        (date(2024, 1, 1), "元日"),
        (date(2024, 1, 8), "成人の日"),
    ]


def test_holidays():
    holidays = yasumi.holidays(date(2024, 1, 1), date(2024, 1, 8))
    assert holidays == [
        (date(2024, 1, 1), "元日"),
        (date(2024, 1, 8), "成人の日"),
    ]


def test_consistency_with_jpholiday():
    between_jpholiday = jpholiday.between(date(1900, 1, 1), date(2100, 1, 1))
    between_yasumi = yasumi.between(date(1900, 1, 1), date(2100, 1, 1))

    assert between_jpholiday == between_yasumi