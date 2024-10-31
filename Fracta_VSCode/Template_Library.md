---
system:
    title: TM_FILENAME_BASE
    size: 
    file_format: $TM_FILENAME
    create_time: $CURRENT_YEAR/$CURRENT_MONTH$CURRENT_DATE/$CURRENT_HOUR:$CURRENT_MINUTE/$CURRENT_TIMEZONE_OFFSET
    last_edit_time: $CURRENT_YEAR/$CURRENT_MONTH$CURRENT_DATE/$CURRENT_HOUR:$CURRENT_MINUTE/$CURRENT_TIMEZONE_OFFSET
    hash:

link:
    all_link:
    # type & level单选：life, focus, random
    structure_link:
    index:
        # height+complexity
        total:
        # the index of difficult,sum of hierarchical
        height:
        # the index of complex, sum of cross
        complexity:
    
flow:
    # date_involved
    Record:

    Statis:
        End_time: 
        Start_time: 
        Total_Cost(¥):
        # flow sum = actual duration
        Total_Duration: 

library:
    category:
        # book, article, paper, podcast, magazine, courser,game, movie, show 
        type: 
        # genres
        genres:
        # index of quality. 1-5
        restriction:
            sex:
            violence:
            age:
    rate:
        amazon:
        IMDB:
        personal:
        recommender:
    creator:
        author:
        first_version:
            editor:
            publisher:
            year:
            # the origin language
            language:
    location:
        local_file:
        online_server:
---
tags: #library, #random

<!-- hint: 
    - 体验过的
    - 准备体验的
    - 看到过，只进行5分钟的了解，而且并不准备进行体验的

内容需要收录两部分
    - 作品信息：作者，主题，出版社等等，能用YAML表示就用YAML
    - 主观体验：游玩记录，感受，评价等recommendation by recommender and link -->
