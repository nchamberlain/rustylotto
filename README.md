# rustylotto
Display historical lotto results on playslips.

## Process

A csv with historical draw numbers for California SuperLotto is downloaded from https://www.lotteryusa.com/california/super-lotto-plus/year. This file has a specific layout and if a new csv-provider is needed then the code that digests the input csv will have to be adjusted.

The downloaded file has the following characteristics:

-   The date in the left column is verbose with the fully spelled-out day of the week plus the date as one would hand write it.

-   The date is parsed into a chrono NaiveDate which is then re-written as YYYY-MM-DD-DAY. This is done because the date will (eventually) become the filename for each chart that will be saved.

-   The "main numbers" (the 5 numbers that you have to match) are enclosed in double-quotes. Inside the quotes, each number is separated by a comma so if you load this csv file into a spreadsheet, the main numbers are all in the same cell. This was done, I'm pretty sure, to make sure that the Mega Number is separate.

-   The main numbers are grabbed as a slice, including the Mega number. This has the disadvantage of capturing the closing quote which is then deleted. The main number and the mega number are then 6 numbers separated by commas. When finished, the first 5 numbers will be displayed in the main area and the last number will be displayed in the mega area.

-   The std file from csv-provider also has a text jackpot field which is not required. Because this program uses slices of the leftmost fields, the jackpot field is ignored. No other processing of it is required.

Assuming that the csv conversion goes OK, the program then loads the playslip png and creates arrays of position tuples that are indexed by the superlotto number (1-47 for main numbers, 1-27 for mega numbers). It then iterates 1..48 over the main numbers array and draws a colored box over the numbers on the playslip. It then iterates 1..28 over the mega numbers and draws different colored boxes ove the playslip number.

## To Do

We now need teach it to read the numbers from the csv_out.csv file and draw boxes over just those numbers. When all 5+1 numbers have been boxed over, the chart needs to be written to a file.

When all 50+ historical draws have been processed, I can then step thru the charts in date order to see the trends and clusters in the numbers drawn.
