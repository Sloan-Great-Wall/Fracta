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

node:
    # 状态(带百分比），表示这个事件的进度。单选：idea, executable, pending, in-progress, done, verified, archived。添加flow数据参考后可自动根据预计用时和实际用时自动计算完成度。
    status:
    # 优先级
    priority:
    person:
    # 效率，对目前进度质量的评价。单选：high, mid, low
    efficiency:
---
tags:

---

WOOP
---