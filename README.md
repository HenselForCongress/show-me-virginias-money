# Show Me Virginia's Money

## Overview

This Rust application is designed to aggressively scrape financial report data from the Virginia election website, overcoming access restrictions and ensuring data availability to the public. It leverages headless Chrome to establish a session and retrieve cookies, and then uses those cookies to perform direct file downloads via `reqwest`.

If the Commonwealth of Virginia wasn't so keen on keeping this data from being used externally, I wouldn't have to be so aggressive. #DefaultOpen

This project *mirrors* the content found on the [official Virginia reports](https://apps.elections.virginia.gov/SBE_CSV/CF/). The only difference is that these files are actually usable. IE you don't have to download one report at a time. Additionally, this code runs every night and stores results in this project. If any of the files change or are modified, then those changes are reflected here.

[View All Virginia Campaign Finance Reports](reports)

## Features

- **Headless Chrome Integration:** Uses headless Chrome to navigate to the Virginia election website, establish a session, and retrieve necessary cookies.
- **Aggressive Downloading:** Uses aggressive request headers to mimic a real browser closely and bypass common access restrictions.
- **Comprehensive Logging:** Provides real-time logging of the scraping process, including successes and failures.
- **File Organization:** Automatically organizes downloaded files into a `reports` directory with subdirectories for each date.


## Usage
### Dependencies
Make sure you have rust installed.

### Running the Application


1. Clone the repository:
   ```sh
   git clone https://github.com/HenselForCongress/show-me-virginias-money.git
   cd show-me-virginias-money
   ```

2. Ensure you have the latest stable version of Rust and Cargo installed.

3. Run the application:
   ```sh
   RUST_LOG=info cargo run
   ```


### Output

- **reports directory:** Contains downloaded files organized by year and month.

### Example

```
reports/
├── 2012_03
│   ├── Report.csv
│   ├── ScheduleA.csv
│   ├── ScheduleB.csv
│   ├── ScheduleC.csv
│   ├── ScheduleD.csv
│   ├── ScheduleE.csv
│   ├── ScheduleF.csv
│   ├── ScheduleG.csv
│   ├── ScheduleH.csv
│   ├── ScheduleI.csv
...
```

## Contributing

Feel free to fork this repository and submit pull requests. Any improvements and suggestions are welcome!


## License

All repositories under the Hensel for Congress organization are licensed under the GNU Affero General Public License version 3.0 (AGPL-3.0). You are free to use, copy, distribute, and modify the software as long as any modifications or derivative works are also licensed under AGPL-3.0. This ensures that the source code remains available to users interacting with the software over a network, promoting transparency and the freedom to modify networked software.

For more details, see the [full text of the license](https://www.gnu.org/licenses/agpl-3.0.html).

<div align="center">
  <table border="1" style="border-collapse: collapse; border: 2px solid black; margin-left: auto; margin-right: auto;">
    <tr>
      <td style="padding: 10px;">
        Paid for by Hensel for Congress
      </td>
    </tr>
  </table>
</div>
