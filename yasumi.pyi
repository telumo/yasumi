from datetime import datetime, date

def is_holiday_name(date: datetime | date) -> str | None:
    """与えられた日付が祝日である場合、祝日名を返します.それ以外の場合はNoneを返します.

    Parameters
    ----------
    date : datetime | date
        祝日かどうかを調べる日付

    Returns
    -------
    str | None
        祝日名またはNone

    Examples
    --------
    >>> from datetime import datetime
    >>> is_holiday_name(datetime(2024, 1, 1))
    '元日'
    >>> is_holiday_name(datetime(2024, 1, 2))
    None

    Notes
    -----
    この関数はholiday_nameと同じです.
    """

def holiday_name(date: datetime | date) -> str | None:
    """与えられた日付が祝日である場合、祝日名を返します.それ以外の場合はNoneを返します.

    Parameters
    ----------
    date : datetime | date
        祝日かどうかを調べる日付

    Returns
    -------
    str | None
        祝日名またはNone

    Examples
    --------
    >>> from datetime import datetime
    >>> holiday_name(datetime(2024, 1, 1))
    '元日'
    >>> holiday_name(datetime(2024, 1, 2))
    None

    Notes
    -----
    この関数はis_holiday_nameと同じです.
    """

def is_holiday(date: datetime | date) -> bool:
    """与えられた日付が祝日である場合、Trueを返します.それ以外の場合はFalseを返します.

    Parameters
    ----------
    date : datetime | date
        祝日かどうかを調べる日付

    Returns
    -------
    bool
        祝日かどうか

    Examples
    --------
    >>> from datetime import datetime
    >>> is_holiday(datetime(2024, 1, 1))
    True
    >>> is_holiday(datetime(2024, 1, 2))
    False
    """

def is_no_workday(date: datetime | date) -> bool:
    """与えられた日付が土日祝である場合、Trueを返します.それ以外の場合はFalseを返します.

    Parameters
    ----------
    date : datetime | date
        土日祝かどうかを調べる日付

    Returns
    -------
    bool
        土日祝かどうか

    Examples
    --------
    >>> from datetime import datetime
    >>> is_no_workday(datetime(2024, 9, 15)) # 9/15は日曜日
    True
    >>> is_no_workday(datetime(2024, 9, 16)) # 9/16は敬老の日
    True
    >>> is_no_workday(datetime(2024, 9, 17))
    False
    """

def month_holidays(year: int, month: int) -> list[tuple[date, str]]:
    """指定された年月の祝日一覧を返します.

    Parameters
    ----------
    year : int
        年
    month : int
        月

    Returns
    -------
    list[tuple[date, str]]
        祝日一覧

    Examples
    --------
    >>> month_holidays(2024, 1)
    [(datetime.date(2024, 1, 1), '元日'), (datetime.date(2024, 1, 8), '成人の日')]
    """

def year_holidays(year: int) -> list[tuple[date, str]]:
    """指定された年の祝日一覧を返します.

    Parameters
    ----------
    year : int
        年

    Returns
    -------
    list[tuple[date, str]]
        祝日一覧

    Examples
    --------
    >>> len(year_holidays(2024))
    21
    """

def holidays(start_date: datetime | date, end_date: datetime | date) -> list[tuple[date, str]]:
    """指定された期間の祝日一覧を返します.

    Parameters
    ----------
    start_date : datetime | date
        開始日(含む)
    end_date : datetime | date
        終了日(含む)

    Returns
    -------
    list[tuple[date, str]]
        祝日一覧

    Examples
    --------
    >>> holidays(datetime(2024, 1, 1), datetime(2024, 1, 31))
    [(datetime.date(2024, 1, 1), '元日'), (datetime.date(2024, 1, 8), '成人の日')]

    Notes
    -----
    この関数はbetweenと同じです.
    """

def between(start_date: datetime | date, end_date: datetime | date) -> list[tuple[date, str]]:
    """指定された期間の祝日一覧を返します.

    Parameters
    ----------
    start_date : datetime | date
        開始日(含む)
    end_date : datetime | date
        終了日(含む)

    Returns
    -------
    list[tuple[date, str]]
        祝日一覧

    Examples
    --------
    >>> between(datetime(2024, 1, 1), datetime(2024, 1, 31))
    [(datetime.date(2024, 1, 1), '元日'), (datetime.date(2024, 1, 8), '成人の日')]

    Notes
    -----
    この関数はholidaysと同じです.
    """