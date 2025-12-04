# Elements 模块文档

## 概述

`elements` 模块定义了 Sho 系统中用于描述历史事件和实体的核心数据结构。该模块分为五个主要类别：

- **核心实体 (Core Entities)**: 人物、团体、物品等基本实体
- **时地 (Temporal & Spatial Context)**: 时间、地点、朝代、年号等上下文信息
- **事件与行为 (Events & Actions)**: 事实、事迹、言论、大事件等动态过程
- **情势与体系 (Situations & Systems)**: 状态、制度、关系等静态结构
- **史料与解读 (Historical Sources & Interpretation)**: 史料来源、文献典籍、评价等

---

## 1. 核心实体 (Entities)

### 1.1 Person (人物)

**文件**: `entities/person.rs`

人物是历史事件中的核心参与者。

**字段说明**:
- `label: String` - 人物标签/名称
- `gender: Option<Gender>` - 性别（可选）
  - `Male` - 男性
  - `Female` - 女性
  - `Other` - 其他
- `surname: Option<String>` - 姓（可选）
- `forename: Option<String>` - 名（可选）
- `pseudonym: Option<Vec<String>>` - 号（可选，可多个）
- `courtesy_name: Option<String>` - 字（可选）
- `other_names: Option<Vec<String>>` - 其他名称（可选）
- `nickname: Option<String>` - 绰号/昵称（可选）
- `birth_time: Option<Time>` - 出生时间（可选）
- `death_time: Option<Time>` - 死亡时间（可选）

### 1.2 Group (团体/势力)

**文件**: `entities/group.rs`

团体、势力、意识形态等组织形式。

**字段说明**:
- `name: String` - 团体名称
- `group_type: GroupType` - 团体类型
  - `Regime` - 政权
  - `MilitaryGroup` - 军事集团
  - `AcademicSchool` - 学术流派
  - `CivilOrganization` - 民间组织
  - `Other(String)` - 其他类型
- `representatives: Option<Vec<Id>>` - 代表人物/领袖（可选）
- `establishment_time: Option<Time>` - 建立时间（可选）
- `end_time: Option<Time>` - 结束时间（可选）
- `core_ideology: Option<String>` - 核心理念/宗旨（可选）
- `main_activity_region: Option<String>` - 主要活动区域（可选）
- `description: Option<String>` - 描述（可选）

### 1.3 Item (物品)

**文件**: `entities/item.rs`

历史事件中涉及的物品、器物等客体。

**字段说明**:
- `name: String` - 物品名称
- `item_type: ItemType` - 物品类型
  - `Weapon` - 兵器
  - `Mount` - 坐骑
  - `Treasure` - 宝物
  - `Other(String)` - 其他类型
- `owners: Option<Vec<Ownership>>` - 所有者（可选，可为多个并注明时间）
  - `Ownership` 结构包含：
    - `owner: Id` - 所有者ID
    - `start_time: Option<Time>` - 开始拥有时间
    - `end_time: Option<Time>` - 结束拥有时间
- `origin: Option<String>` - 出处/来源（可选）
- `description: Option<String>` - 描述（外观、功能、相关典故等，可选）

---

## 2. 时地 (Context)

### 2.1 Time (时间)

**文件**: `context/time.rs`

时间表示系统，支持精确时间、粗略时间和时间范围。

**类型结构**:
- `Time` - 时间枚举
  - `Timepoint(Timepoint)` - 时间点
  - `TimeRange(TimeRange)` - 时间范围

- `Timepoint` - 时间点枚举
  - `ExactTime(ExactTime)` - 精确时间
  - `RoughTime(RoughTime)` - 粗略时间

- `ExactTime` - 精确时间
  - `year: Year` - 年份（`i32` 类型）
  - `month_day: MonthDay` - 月日
    - `month: Option<u32>` - 月份（可选）
    - `day: Option<u32>` - 日期（可选）

- `RoughTime` - 粗略时间
  - `Century(i8)` - 世纪

- `TimeRange` - 时间范围枚举
  - `BetweenFact(Id, Id)` - 两个事实之间
  - `BetweenTime(Timepoint, Timepoint)` - 两个时间点之间

### 2.2 Location (地点)

**文件**: `context/location.rs`

**警告**: 地理方面方案还未确定

**字段说明**:
- `name: String` - 地点名称
- `ancient_names: Option<Vec<String>>` - 别称/古称（可选）
- `location_type: LocationType` - 地点类型
  - `Prefecture` - 州郡
  - `City` - 城池
  - `Pass` - 关隘
  - `Battlefield` - 战场
  - `Mountain` - 山川
  - `River` - 河流
  - `Other(String)` - 其他类型
- `geographic_description: Option<String>` - 地理位置描述/大致坐标（可选）
- `historical_evolution: Option<String>` - 历史沿革/描述（可选，如：某地在不同时间属不同势力管辖）

### 2.3 Dynasty (朝代)

**文件**: `context/dynasty.rs`

界定历史事件发生的朝代背景。

**字段说明**:
- `name: String` - 朝代名称
- `start_bc_year: Option<u32>` - 起始年份（公元前，可选）
- `first_motto: Option<Id>` - 最早的年号（可选，起到类似链表头的作用）

### 2.4 ReignMotto (年号)

**文件**: `context/reign_motto.rs`

界定历史事件发生的年号背景。

**字段说明**:
- `name: String` - 年号名称
- `emperor: Id` - 皇帝ID
- `dynasty: Id` - 朝代ID
- `start_time: Option<Time>` - 开始时间（可选）
- `end_time: Option<Time>` - 结束时间（可选）
- `previous_reign_motto: Option<Id>` - 上一个年号（可选）
- `next_reign_motto: Option<Id>` - 下一个年号（可选）

---

## 3. 事件与行为 (Events)

### 3.1 Fact (原子事件)

**文件**: `events/fact.rs`

有明确起止的动态过程，是最基本的事件单元。

**字段说明**:
- `time: Time` - 时间
- `related: Vec<Id>` - 相关实体ID列表
- `content: String` - 事件内容描述
- `location: Option<Id>` - 地点ID（可选，预留给地点功能）
- `same: Option<Id>` - 重复事件ID（可选，预留给重复事件）

### 3.2 Deed (事迹)

**文件**: `events/deed.rs`

人物主动行为，有明确动作。

**字段说明**:
- `label: String` - 事迹标签
- `subject: Id` - 行为主体ID
- `related: Vec<Id>` - 相关实体ID列表
- `content: String` - 事迹内容描述
- `time: Option<Time>` - 时间（可选）
- `result: Option<Id>` - 结果ID（可选）
- `location: Option<Id>` - 地点ID（可选，预留给地点功能）
- `same: Option<Id>` - 重复事件ID（可选，预留给重复事件）

### 3.3 Episode (大事件)

**文件**: `events/episode.rs`

有明确起止的动态过程，由多个原子事件组成。

**字段说明**:
- `time: Time` - 时间范围
- `related: Vec<Id>` - 相关实体ID列表
- `sub_facts: Vec<Id>` - 子事实ID列表（组成该大事件的原子事件）
- `result: Id` - 表示结果的Fact的ID
- `same: Option<Id>` - 重复事件ID（可选，预留给重复事件）

### 3.4 Utterance (言论)

**文件**: `events/utterance.rs`

人物直接或转述的言论。

**字段说明**:
- `subject: Id` - 言论主体ID
- `content: String` - 言论内容
- `time: Option<Time>` - 时间（可选）
- `related: Vec<Id>` - 相关实体ID列表
- `source: Id` - 来源ID（指向史料节点）

**注意**: 考虑添加语境字段

---

## 4. 情势与体系 (Structures)

### 4.1 State (状态)

**文件**: `structures/state.rs`

某时某地的静态情形。

**当前状态**: 目前没有好设计，先预留

### 4.2 Institution (制度)

**文件**: `structures/institution.rs`

政治/军事/经济规则。

**字段说明**:
- `label: String` - 制度标签
- `subject: Id` - 制度主体ID（实施该制度的实体）
- `content: String` - 制度内容描述
- `start_time: Option<Time>` - 开始时间（可选）
- `source: Id` - 来源ID（指向史料节点）

### 4.3 Relation (关系)

**文件**: `structures/relation.rs`

实体间的持久关联。

**字段说明**:
- `subject: Id` - 关系主体ID
- `object: Vec<Id>` - 关系对象ID列表
- `relationship: RelationShip` - 关系类型
  - `Parent` - 父母
  - `Child` - 子女
  - `Sibling` - 兄弟姐妹
  - `Cousin` - 堂/表亲
  - `FellowTown` - 同乡
  - **注意**: 关系分类尚未完成（TODO）
- `relation_text: String` - 具体描述（如选择Sibling时可以备注具体关系）
- `time: Option<Time>` - 时间（可选）
- `description: String` - 关系描述

---

## 5. 史料与解读 (Sources)

### 5.1 Source (史料/来源)

**文件**: `sources/source.rs`

历史研究的原始材料及其来源。

**字段说明**:
- `name: String` - 名称（如：《三国志·卷三十六·蜀书六·关羽传》）
- `literature: Option<Id>` - 所属典籍（可选，引用自"文献典籍"元素）
- `author: Vec<Author>` - 作者列表
- `completion_time: Option<Time>` - 成书年代（可选）
- `source_nature: Option<SourceNature>` - 史料性质（可选）
  - `PrimarySource` - 一手史料
  - `SecondarySource` - 二手史料
  - `OfficialHistory` - 官方史书
  - `UnofficialHistory` - 野史笔记
  - `Other(String)` - 其他类型
- `reliability_assessment: Option<String>` - 可靠性评估（可选，简要描述其价值与局限性）

### 5.2 Literature (文献典籍)

**文件**: `sources/literature.rs`

历史研究的原始文献材料。

**字段说明**:
- `name: String` - 文献名称
- `alias: Option<Vec<String>>` - 别名（可选）
- `authors: Vec<Author>` - 作者列表（可为多个或"佚名"）
- `completion_time: Option<Time>` - 成书时间（可选）
- `literature_type: LiteratureType` - 文献类型
  - `History` - 史书
  - `Philosophy` - 子部（经史子集中的子）
  - `Military` - 兵书
  - `Literature` - 文学作品
  - `Other(String)` - 其他类型
- `summary: Option<String>` - 主要内容/摘要（可选）
- `preservation_status: Option<PreservationStatus>` - 流传情况（可选）
  - `Complete` - 完本
  - `Fragmentary` - 残本
  - `Lost` - 已佚

### 5.3 Assessment (评价)

**文件**: `sources/assessment.rs`

史家或时人的价值判断。

**字段说明**:
- `subject: Id` - 评价主体ID
- `object: Id` - 评价对象ID
- `content: String` - 评价内容
- `origin: Id` - 来源ID（应指向史料节点）

### 5.4 Author (作者信息)

**文件**: `sources/author.rs`

作者信息枚举类型。

**类型**:
- `Named(Id)` - 有名作者（ID指向Person）
- `Anonymous` - 佚名

---

## 模块导出

`elements/mod.rs` 重新导出了所有子模块的公共类型，可以直接从 `elements` 模块使用：

```rust
use sho_core::elements::*;
```

---

## 设计说明

### 时间系统

时间系统采用分层设计：
- 支持精确时间（年月日）和粗略时间（世纪）
- 支持时间范围，可以通过两个事实或两个时间点定义
- 年份使用 `i32` 类型，支持公元前和公元后

### ID 引用系统

所有元素通过 `Id` 类型进行相互引用，形成知识图谱结构。这种设计允许：
- 实体之间的灵活关联
- 避免数据重复
- 支持复杂的查询和推理

### Builder 模式

大部分元素类型使用了 `bon::Builder` 派生宏，支持流畅的构建模式：

```rust
let person = Person::builder()
    .label("刘备".to_string())
    .surname(Some("刘".to_string()))
    .build();
```

### 预留字段

部分元素包含 `same` 字段（如 `Fact`、`Deed`、`Episode`），用于处理重复事件的识别。`location` 字段在多个事件类型中预留，等待地点功能的完善。

---

## 待完善功能

1. **地点系统**: 地理方面方案还未确定（见 `Location` 的警告）
2. **状态设计**: `State` 类型目前为空，需要进一步设计
3. **关系分类**: `RelationShip` 枚举尚未完成（标记为 TODO）
4. **言论语境**: `Utterance` 考虑添加语境字段
