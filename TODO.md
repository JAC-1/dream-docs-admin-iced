 # Iced ui

- [ ] Test text-input with no value change and see if it can be copy and pasted
	- [ ] If it works use it for copy and pastable text to skip some of the text setup till later

- [ ] Refactor navbar
	- [ ] Classes are displayed in home
	- [ ] Students are rows of students (columns = ["Kanji", "kana", "English", "Class", "Program"])
		- [ ] Has a input bar for future search
		- [ ] Clicking on a student displays the student page

- [ ] Test out events, like mouse over
	- [ ] Final column in Students will have colored circles labeled "tasks"
	- [ ] Colors = ["no fill(not submitted)", "orange(pending)", "green(approved)", "red(declined)"]
	- [ ] Each circle is clickable
	- [ ] Clicking will eventually open the file or download it
	- [ ] Hovering will show a bottom right of the location of the mouse cursor

- [ ] Student page
	- [ ] One big container with one column
	- [ ] Two rows in the column
	- [ ] Row(1) has two columns - col(1) has Kanji name and other information; col(2) is empty for now
	- [ ] Row(2) has a table:
		- [ ] Column of 2 rows
		- [ ] Row(1) is used to position a "download all" button
		- [ ] Row(2) contains a table made of multiple rows and columns
			- [ ] Each row has ["document", "date submitted", "status", "download"(will be changed to view once I figure that out), "Decline button with red text", "Green Approve button"]
			- [ ] Rows for each document type

- [ ] Set up proper message passing between components
- [ ] Implement proper error handling for all UI interactions
- [ ] Add loading states for async operations
- [ ] Set up proper styling system using Iced's built-in styling
- [ ] Implement proper state management
- [ ] Add keyboard shortcuts for common actions
