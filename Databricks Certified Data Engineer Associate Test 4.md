---
date: "2025-05-16"
datetime: "2025-05-16 20:21:45"
book: 
page: 
tags: 
references: 
aliases:
---
## Q16
You are asked to setup two tasks in a databricks job, the first task runs a notebook to download the data from a remote system, and the second task is a DLT pipeline that can process this data, how do you plan to configure this in Jobs UI
#### A16
- Single job can be used to setup both notebook and DLT pipeline, use two different tasks with linear dependency.
- Single Job을 사용해 notebook, DLT pipline 모두 설정 가능
	- 선형 종속성을 가진 두가지 작업 사용 가능
	- notebook task -> DLT task

## Q17
You are asked to set up an alert to notify in an email every time a KPI indicater increases beyond a threshold value, team also asked you to include the actual value in the alert email notification.
#### A17
- Setup an alert but use the custom template to notify the message in email’s subject
- Custom template을 사용하여 변수 사용이 가능함

## Q18
Operations team is using a centralized data quality monitoring system, a user can publish data quality metrics through a webhook, you were asked to develop a process to send messages using a webhook if there is atleast one duplicate record, which of the following approaches can be taken to integrate an alert with current data quality monitoring system
#### A18
- Setup an alert with custom Webhook destination
- Alert Destination에 webhook이 존재함

## Q19
You are currently working with the application team to setup a SQL Endpoint point, once the team started consuming the SQL Endpoint you noticed that during peak hours as the number of concurrent users increases you are seeing degradation in the query performance and the same queries are taking longer to run, which of the following steps can be taken to resolve the issue?
#### A19
- They can increase the maximum bound of the SQL endpoint’s scaling range.

## Q20
The data engineering team is using a bunch of SQL queries to review data quality and monitor the ETL job every day, which of the following approaches can be used to set up a schedule and automate this process?
#### A20
- They can schedule the query to refresh every 1 day from the query’s page in Databricks SQL.
- query info 탭에서 refresh schedule 지정 가능
