---
语言：zh-CN
标题：简介
---

# 智会平台数据库（API部分）

首先，我们使用`mongodb`来管理数据，所有的数据都存储在`zhihui-basicdata`数据库中。

## 集合

1. ### `Classrooms`
    这是教室的集合。它包含以下字段：
    - `gradeid`：教室的年级ID。
    - `classid`：教室的班级ID。
    - `password`：教室密码，不仅用`base64`加密，还用`sha-512`加密。

2. ### `Admins`
    这是管理员的集合。它包含以下字段：
    - `username`：管理员的用户名。
    - `password`：admin 的密码，不仅用`base64`加密，还用`sha-512`加密。
    - `auths`：管理员可以管理的部分。

3. ### `Member`
    这是成员的集合。它包含以下字段：
    - `basic_data`：成员的基本数据。
        - `username`：成员的用户名。
        - `number`：成员的编号。
        - `password`：会员的密码，不仅用`base64`加密，还用`sha-512`加密。

4. ### `Deduction`
    这是扣分的集合。它包含以下字段：
    - `deductionid`：扣分的ID。
    - `person`：被扣分的人。
    - `reason`：扣分的原因，使用**扣分原因编号**等记。
    - `deduction`：扣分的扣分。
    - `time`：扣减的时间。
    - `deductor`：扣分的人。
    - `desc`：扣减的描述。

5. ### `Tasks`
    这是任务的集合。它包含以下字段：
    - `taskid`：任务ID。
    - `title`：任务的标题。
    - `description`：任务的描述。
    - `deadline`：任务的截止日期。
    - `status`：任务的状态，如下，
        - `person`：人。
        - `status`：状态，如下：`finished`，`incomplete`，`to-do`。
    - `assigner`：分配任务的人。
    - `assign_time`：分配任务的时间。
    - `finish_time`：任务完成的时间。
    - `finisher`：监督他们完成任务的人。
    - `comment`：任务的评论。
    - `volunteer`：如果可以注册志愿时间，将作为时间输入。

6. ### `Posts`
    这是稿件的集合。它包含以下字段：
    - `postid`：稿件的ID。
    - `title`：稿件的标题。
    - `content`：稿件的内容，`markdown`。
    - `author`：稿件的作者。
    - `time`：发帖时间。
    - `tags`：稿件的标签，使用数组。
    - `comment`：稿件的评论。