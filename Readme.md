# GitHub classroom to Google sheets integration

This is pre-release version of the GitHub classroom to Google sheets integration. 

Aim of the project is to automatically send the assignment results to the Google sheets.

## How to use

1. Get Google sheet API credentials
   1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
   2. Create a new project
   3. Enable Google Sheets API for the project
   4. Create a service account
   5. Download the credentials file
2. Create a new Google sheet
3. Make at least one sheet with name `Sheet1` and a column with name `github_id` (this will be changed in future versions) 
4. Share the sheet with the service account email
5. In your organisation make these secrets available (you can use your names):
   1. `SERVICE_EMAIL` - email of service account you've created in step 1.4
   2. `PRIVATE_API_KEY` - private key from the credentials file (make sure to copy and paste it as it was in the file, with `\n` symbols)
   3. `GOOGLE_SHEET_ID` - the ID of the Google sheet (string from the URL: `https://docs.google.com/spreadsheets/d/<GOOGLE_SHEET_ID>/edit`)
6. In your GitHub actions workflow add the following step:
   ```yaml
    - name: Update Google Sheets with Task01 results
      uses: SPGC/ClassroomToSheetsIntegration@master
      with:
        student-name: "${{ github.actor }}"
        robot-email: "${{ secrets.SERVICE_EMAIL }}"
        private-api-key: "${{ secrets.PRIVATE_API_KEY }}"
        task-results: "${{ steps.task01.outputs.result }}"
        table-id: "${{ secrets.GOOGLE_SHEET_ID }}" 
   ```
    Instead of `task01` you should use name that you've specified as `id` in step that uses `classroom-resources/autograding-command-grader@v1`. 
Other graders are not tested yet, but you can try to use them as well (I hope they should work). 

## Google sheet structure
The action will publish results to the Google sheet in the following format:

| github_id   | task01 | task02 | ...   |
|-------------|--------|--------|-------|
| student_id1 | 0      | 1      | ----- |
| student_id2 | 1      | 1      | ----- |

The `github_id` column is mandatory and should be present in the sheet. The other columns will be created automatically 
based on the tasks that are being graded. You can also make your own columns with additional information. The action will 
automatically find the columns with the task results and update them and if the column does not exist it will create it
(in the first empty column of the sheet).
The same applies to the rows, if the student is not present in the sheet, the action will create a new row for the student.


## How to contribute

TBA