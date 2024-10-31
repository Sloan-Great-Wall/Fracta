# 功能点描述及重要性排序

## file as note as database 
### file with note
each file has a corresponding note file which contain tag head and content body 
each folder has a corresponding note file which contain tag head and content body which is a statis of each file's tag head in csv database formate
in frcat brain finder, note will show when you tap a file or folder, you can talk and mark any file and folder without to open a file in its formate,

file in a folder
    file as a indivudial note
    but parallel file's tag head each as a column data in a database
    prant folder also has its note, its tag head is indivudial, 
    its content body has a part as integrate database of sub file
    
    folder's note(same name, tag head+csv content+note)
    folder
        file 1
        file 1's note
        file 2.md
        ```
        file n
        file n'note

folder in a file
    file as a indivudial note
    when user insert a note/database/file to a file, a folder with a same name will be create,
    this file will become a side note 
    the insert note/database/file will create in that folder

    file
    file's note
    file folder
        file 1
        file 1's note
        file 2.md
        ```
        file n
        file n'note
        folder 1
        folder 1's note
    
    or

    file.md
    file folder
        file 1
        file 1's note
        file 2.md
        ```
        file n
        file n'note
        folder 1
        folder 1's note
    
file in a file
    same as folder in a file

folder in a folder
    same as file in a folder but with a side note

 - [ ]文件系统的命名问题:同种文件不能拥有相同名称

### Template：文档模版，文本块模版，Snippet, veriable
文档模版的两种方式
    搜索模版库，创建新文档，通过snippet添加内容，并且自动触发variable
    搜索模版库，复制模版文档到当前位置，然后识别模版文档中的variables，替换成想要的字段
文本块模版的创建方式
    搜索snippet库，添加内容并自动触发variable
全局模版和数据库模版
    全局模版存在根目录模版库中
    数据库模版是一种默认设置，当一个folder中，有“notetemplate:"开头的文件时，在这个folder内会自动根据此创建。
    除非使用搜索模版库的方式创建文件

## 字段级别的wikilink
### 文档：双向链接，双向显示
wikilink: 通过[[]]或其他标识，链接一个file，显示标题。被链接文档会显示连接他的文档名，backlink。当鼠标移到链接或被链接的标识上时，会显示那个文档的信息
wikilink会实时同步更新，比如更改链接的文档文件名，改变文档的内容，改变文档的位置

当通过标识创建一个不存在的文件的链接时，这个链接的文件名会出现在placeholder区，这里是一个单独的文件夹，以链接中的文件名建立的note空文件会放在这里。 或者 创建wikilink时，会在全局文件中选择，当链接的是一个不存在的文件时，需要选择路径，创建一个新文件
  
foam:
    backlink会出现在all link中
    YAML中的link不会出现在back all link中

### 字段块：内容同步
在正文内容和标签头中，复制一个内容块到另一个note文件中，以同步方式粘贴。两边的内容便会实时同步，更改一处，另一处也会随之改变

- [x] 直接打开一个vscode fold，开始编辑，有一样的视图
    - [x] embed一个文件的block到另一个文件中，是否正确显示内容
      - [x] 是否能单向同步编辑，
      - [ ] 是否能双向同步编辑
    - [ ] 是否能跨文件embed
      - [ ] 可向下跨文件embed，向上只能wikilink

### content & link graph：目录与知识图谱
将markdown各级title转化成可toggle的悬浮目录
将wikilink转化成可视图谱
    可调节的显示范围，比如只显示标签头中woop部分地关系

## 文本与多媒体内容：嵌入，显示，排版

### 显示与排版
解释器
    图片，视频，网页
CSS

### 正文：Markdown+latex+CSV
自然语言 Markdown
    headerx
    list
    bold
    italic
    table
    code
    image
    file
数学语言 latex
    mathblock
内部向量数据库 CSV

### 向量标签：YAML

添加YAML表头到markdown中，并添加标签识别

- [ ]  YAML在表头会不显示，能否选择显示范围
- [x]  YAML中是否能正确显示标签
- [x]  YAML中结构标签是否能正确显示
- [x]  多级YAML能否显示

### 日历会议联系人

### 其他笔记文件兼容

CSV，markdown

notion

onenote

### 多媒体文件嵌入，网络应用嵌入embed，视图与排版

富文本嵌入
    本地文件：图片，音频，视频，网页
        通过vscode可直接拖入
    在线链接：通过embed可嵌入超链接

其他应用嵌入

日历

figjam

github

view 视图

board，list，gallery

如何用脚本自动生成视图：脚本语言，脚本

排版

vscode的字体

markdown排版

## 多个不同身份账户的实时编辑与版本控制

### 离线控制，版本分支控制，云盘存取多人协作

文件离线同步

OneDrive

版本控制

github
vscode plugin gitdoc

多端使用

MAC，IOS，Window端编辑器

与icloud结合，双向同步数据到apple calendar，reminder等

### 网络发布
gitpage
mirror

## AI原生与自动化
 + 自动化执行
脚本语言，脚本
自动总结文档内容

链接结构化数据和非结构化数据

生成Tag，交给自动化

# 使用场景及userflow
### flow日记 与 时间段规划

用snippet或插件自动加当前时间

- [ ]  insertdatetime
- [ ]  insertdate
- [ ]  inserttime
- [ ]  insertsecondunix

Vscode 原生 snippet

```markdown
For inserting the current date and time:

CURRENT_YEAR The current year
CURRENT_YEAR_SHORT The current year's last two digits
CURRENT_MONTH The month as two digits (example '02')
CURRENT_MONTH_NAME The full name of the month (example 'July')
CURRENT_MONTH_NAME_SHORT The short name of the month (example 'Jul')
CURRENT_DATE The day of the month as two digits (example '08')
CURRENT_DAY_NAME The name of day (example 'Monday')
CURRENT_DAY_NAME_SHORT The short name of the day (example 'Mon')
CURRENT_HOUR The current hour in 24-hour clock format
CURRENT_MINUTE The current minute as two digits
CURRENT_SECOND The current second as two digits
CURRENT_SECONDS_UNIX The number of seconds since the Unix epoch
CURRENT_TIMEZONE_OFFSET The current UTC time zone offset as +HHMM or -HHMM (example -0700).
```

foam有内置优化版，CURRENT_ 换成 FOAM_DATE_

```markdown
For inserting the current date and time:

$FOAM_DATE_YEAR The current year
$FOAM_DATE_YEAR_SHORT The current year's last two digits
$FOAM_DATE_MONTH The month as two digits (example '02')
$FOAM_DATE_MONTH_NAME The full name of the month (example 'July')
$FOAM_DATE_MONTH_NAME_SHORT The short name of the month (example 'Jul')
$FOAM_DATE_DATE The day of the month as two digits (example '08')
$FOAM_DATE_DAY_NAME The name of day (example 'Monday')
$FOAM_DATE_DAY_NAME_SHORT The short name of the day (example 'Mon')
$FOAM_DATE_HOUR The current hour in 24-hour clock format
$FOAM_DATE_MINUTE The current minute as two digits
$FOAM_DATE_SECOND The current second as two digits
$FOAM_DATE_SECONDS_UNIX The number of seconds since the Unix epoch
$FOAM_DATE_TIMEZONE_OFFSET The current UTC time zone offset as +HHMM or -HHMM (example -0700).
```

创建时间，上次编辑等时间信息需要外部脚本同步

### Node任务 与 知识网络

### Matter数据库 与 结构化信息
# Start Guide 使用指南
intro
    markdown编辑

prepare
    vscode
    markdown
    foam link

feature
    file is note
    wikilink
        link
        backlink
        graph
        tag
        orphan
    template
        snippet 与多位数据库
    version

fracta brain
    flow
        flow正文格式
    node
        node正文格式
    library
        单一数据
        文件夹
        文件夹综合视图

## ios端使用流程
输入-编辑-输出
idea： 打开bear新建文件，编辑，保存到onedrive
添加：打开file，导航到onedrive中对应文件，编辑，保存到onedrive中

# 案例：搭建Random Library

标签头组块

预设模版

用模版添加内容

# 案例：个人数据系统

个人数据系统架构梳理
    flow记录的模版问题
    周期性任务的模版问题
    数据库library的模版结构问题
    数据格式转化脚本
        这两天的flow 研究标准格式
        notion flow转flow+其他
        notion node转执行标签 + 双莲
    密码-帐号
        onenote敏感数据迁移
        核心密码记录
    数据-设备
        onenote移动本地
        notion vision部分移到本地
        notion结构和onedrive结构对应梳理