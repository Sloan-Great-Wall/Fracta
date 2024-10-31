---
system:
    # origin_title.中文标题
    title: $FOAM_TITLE
    size: 
    file_format: markdown
    time_zone: $CURRENT_TIMEZONE_OFFSET
    create_time: $FOAM_DATE_YEAR/$FOAM_DATE_MONTH$FOAM_DATE_DATE/$FOAM_DATE_HOUR$FOAM_DATE_MINUTE
    last_edit_time: $FOAM_DATE_YEAR/$FOAM_DATE_MONTH$FOAM_DATE_DATE/$FOAM_DATE_HOUR$FOAM_DATE_MINUTE
flow:
    # 以下三个tag中有任意两个都可以确定另外一个
    start_time: 
    end_time: 
    duration(mins): 
    # 记录金钱的账面支出与收入，参考损益表而不是现金流表
    cost: 0$

todo:
    # 状态，表示这个事件的进度。单选：idea, executable, pending, in-progress, done, verified, archived。添加flow数据参考后可自动根据预计用时和实际用时自动计算完成度
    status:
    # 效率，对目前进度质量的评价。单选：high, mid, low
    efficiency:
GTD:
    # 单选：evolution_hacker, coupling, random
    type:
    wish:
    path:
    operator:
    priority:
    level:
reference:
    # read time = flow sum = actual duration
    actual_duration:
    precentage:
    date_involved:
    Hash:
    link_file:
subtag:
    # keyword, genres, theme, location, recommender
    - library
    - random
random_library:
    # book, article, paper, magazine, courser, movie, show, podcast, game
    random_type:
    # the origin language
    language:
    # index of quality
    rate:
    # sex * violence
    age_limit:
    # the index of difficult
    difficulty:
    creator:
        author:
        publisher:
    create_year:
        first_year: 
    file_path:
---
<!-- hint: 
    - 体验过的
    - 准备体验的
    - 看到过，只进行5分钟的了解，而且并不准备进行体验的

内容需要收录两部分
    - 作品信息：作者，主题，出版社等等，能用YAML表示就用YAML
    - 主观体验：游玩记录，感受，评价等recommendation by recommender and link -->