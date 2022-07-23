# Database (API Part)

Firstly, we use `mongodb` to explore datas, and all of the data is in `zhihui-basicdata` database.

3. ### Members
    This is the collection of members. It contains the following fields:
    - `basic_data`: The basic data of the member.
        - `username`: The username of the member.
        - `number`: The number of the member.
        - `password`: The password of the member, and it is encrypted by not only `base64`, but also `sha-512`.
4. ### Deductions
    This is the collection of deductions. It contains the following fields:
    - `deductionid`: The id of the deduction.
    - `person`: The person who get the deduction.
    - `reason`: The reason of the deduction, following **deduction reason id**.
    - `deduction`: The deduction of the deduction.
    - `time`: The time of the deduction.
    - `deductor`: The person who deduct the deduction.
    - `desc`: The description of the deduction.
5. ### Tasks
    This is the collection of tasks. It contains the following fields:
    - `taskid`: The task id of the task.
    - `title`: The title of the task.
    - `description`: The description of the task.
    - `deadline`: The deadline of the task.
    - `status`: The status of the task, and it is following,
        - `person`: The person.
        - `status`: The status, and it is following: `finished`, `incomplete`, `to-do`.
    - `assigner`: The person who assign the task.
    - `assign_time`: The time of the task is assigned.
    - `finish_time`: The time of the task is finished.
    - `finisher`: The person who has monitored that they have finished the task.
    - `comment`: The comment of the task.
    - `volunteer`: If it is able to register volunteer time, it will be inputed as the time.
6. ### Posts
    This is the collection of posts. It contains the following fields:
    - `postid`: The post id of the post.
    - `title`: The title of the post.
    - `content`: The content of the post, following `markdown`.
    - `author`: The author of the post.
    - `time`: The time of the post.
    - `tags`: The tags of the post, array.
    - `comment`: The comment of the post.
