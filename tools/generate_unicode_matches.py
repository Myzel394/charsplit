#!/usr/bin/python

import requests
from bs4 import BeautifulSoup, element
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

@dataclass(frozen=True)
class Range:
    start: int
    end: int

def extract_raw_ranges_from_header_labeled_table(table: element.Tag) -> list[int]:
    ranges: list[int] = []

    for row in table.find_all("tr"):
        tds = row.find_all("td")

        if len(tds) == 0:
            continue

        raw_index = tds[0].text
        
        index = int(raw_index[2:], 16)

        ranges.append(index)

    return ranges

def get_header_from_matrix_table(table: element.Tag) -> element.Tag:
    return table.find_next("a")

def extract_raw_ranges_from_table_matrix(table: element.Tag) -> list[int]:
    ranges: list[int] = []

    for row in table.find_all("tr")[2:-1]:
        tds = row.find_all("td")

        offset_td, *values_tds = tds

        offset = int(offset_td.text[2:-2] + "0", 16)

        values_td: element.Tag
        for index, value_td in enumerate(values_tds):
            if value_td.text.strip() == "":
                continue

            value = offset + index

            ranges.append(value)

    return ranges

def extract_ranges_from_tr_table(table: element.Tag) -> list[int]:
    ranges: dict[str, set[Range]] = defaultdict(set)

    current_description = None

    start_index = None
    previous_index = None

    for row in table.find_all("tr")[1:-1]:
        tds = row.find_all("td")

        if tds[1].text.strip() == "U+007F":
            # Special case for U+007F DELETE
            ranges[""] = {Range(0x007F, 0x007F)}

            ranges[current_description].add(Range(start_index, previous_index))
            start_index = None
            previous_index = None
            continue

        if tds[0].get("rowspan") is not None:
            if start_index is not None:
                ranges[current_description].add(Range(start_index, previous_index))
                start_index = None
                previous_index = None

            current_description = tds[0].text.strip().replace("\n", " ")
            tds = tds[1:]

        if current_description is None:
            raise ValueError("No description found")

        offset_td = tds[0]

        index = int(offset_td.text[2:], 16)

        if start_index is None:
            start_index = index
            previous_index = index
        elif index == previous_index + 1:
            previous_index = index
        else:
            ranges[current_description].add(Range(start_index, previous_index))
            start_index = index
            previous_index = index

    ranges[current_description].add(Range(start_index, previous_index))

    # Delete empty ranges
    for key in list(ranges.keys()):
        if len(ranges[key]) == 0:
            del ranges[key]

    return ranges

def normalise_ranges(ranges: list[int]) -> list[Range]:
    normalised: list[Range] = []

    start_index = ranges[0]
    previous_index = ranges[0]

    for index in ranges[1:]:
        if index == previous_index + 1:
            previous_index = index
        else:
            normalised.append(Range(start_index, previous_index))
            start_index = index
            previous_index = index

    normalised.append(Range(start_index, previous_index))

    return normalised

def get_ranges_from_header_labeled(tables: list[element.Tag]) -> dict[str, list[Range]]:
    points = defaultdict(list)

    for table in tables:
        header = table.find_previous("h2").find("span")
        raw_ranges = extract_raw_ranges_from_header_labeled_table(table)
        ranges = normalise_ranges(raw_ranges)

        points[header.text].extend(ranges)

    return points

def get_ranges_from_matrix(tables: list[element.Tag]) -> dict[str, list[Range]]:
    points = defaultdict(list)

    for table in tables:
        header = get_header_from_matrix_table(table)
        raw_ranges = extract_raw_ranges_from_table_matrix(table)
        ranges = normalise_ranges(raw_ranges)

        points[header.text].extend(ranges)

    return points

def get_ranges_from_tr_labeled(tables: list[element.Tag]) -> dict[str, list[Range]]:
    points = defaultdict(list)

    for table in tables:
        header = (table.find_previous("h3") or table.find_previous("h2")).find("span")
        ranges = extract_ranges_from_tr_table(table)

        for description, ranges in ranges.items():
            full_header = f"{header.text} ({description})" if description != "" else header.text

            points[full_header].extend(ranges)

    return points

def main() -> None:
    raw_html: str = requests.get('https://en.wikipedia.org/wiki/List_of_Unicode_characters').text

    soup = BeautifulSoup(raw_html, 'html.parser')

    tables = soup.find("main", {"id": "content"}).find_all('table')
    table_tr_labeled: list[element.Tag] = tables[2:8]
    table_header_labeled: list[element.Tag] = tables[8:12] + [tables[13]] + [tables[33]] + tables[46:48] + [tables[50]]
    table_matrix: list[element.Tag] = [tables[12]] + tables[14:33] + tables[34:46] + tables[48:50] + tables[51:68]

    points = {}
    points.update(get_ranges_from_header_labeled(table_header_labeled))
    points.update(get_ranges_from_matrix(table_matrix))
    points.update(get_ranges_from_tr_labeled(table_tr_labeled))

    path = Path("../src/utf8_ranges.rs")

    text = """
pub mod utf8_ranges {
    fn get_group(value: &u32) -> String {
        return (match value {
"""

    for description, ranges in points.items():
        for r in ranges:
            text += f'            {r.start}..={r.end} => "{description}",\n'

    text += """
        }).to_string()
    }
}
"""

    path.write_text(text)

if __name__ == '__main__':
    main()

