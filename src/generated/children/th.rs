// 🤖 This file is generated!

use crate::*;
/// The `<th>` element's children.
#[derive(Clone)]
pub enum TableHeaderChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    DescriptionList(DescriptionList),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Main(Main),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Menu(Menu),
    Metadata(Metadata),
    Meter(Meter),
    Navigation(Navigation),
    NoScript(NoScript),
    Object(Object),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Table(Table),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for TableHeaderChild {
    fn from(child: Abbreviation) -> Self {
        TableHeaderChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for TableHeaderChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        TableHeaderChild::Abbreviation(builder.build())
    }
}
impl From<Address> for TableHeaderChild {
    fn from(child: Address) -> Self {
        TableHeaderChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for TableHeaderChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        TableHeaderChild::Address(builder.build())
    }
}
impl From<Anchor> for TableHeaderChild {
    fn from(child: Anchor) -> Self {
        TableHeaderChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for TableHeaderChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        TableHeaderChild::Anchor(builder.build())
    }
}
impl From<Article> for TableHeaderChild {
    fn from(child: Article) -> Self {
        TableHeaderChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for TableHeaderChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        TableHeaderChild::Article(builder.build())
    }
}
impl From<Aside> for TableHeaderChild {
    fn from(child: Aside) -> Self {
        TableHeaderChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for TableHeaderChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        TableHeaderChild::Aside(builder.build())
    }
}
impl From<Audio> for TableHeaderChild {
    fn from(child: Audio) -> Self {
        TableHeaderChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for TableHeaderChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        TableHeaderChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for TableHeaderChild {
    fn from(child: BidirectionalIsolate) -> Self {
        TableHeaderChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for TableHeaderChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        TableHeaderChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for TableHeaderChild {
    fn from(child: BidirectionalOverride) -> Self {
        TableHeaderChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for TableHeaderChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        TableHeaderChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for TableHeaderChild {
    fn from(child: BlockQuote) -> Self {
        TableHeaderChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for TableHeaderChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        TableHeaderChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for TableHeaderChild {
    fn from(child: Bold) -> Self {
        TableHeaderChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for TableHeaderChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        TableHeaderChild::Bold(builder.build())
    }
}
impl From<Button> for TableHeaderChild {
    fn from(child: Button) -> Self {
        TableHeaderChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for TableHeaderChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        TableHeaderChild::Button(builder.build())
    }
}
impl From<Canvas> for TableHeaderChild {
    fn from(child: Canvas) -> Self {
        TableHeaderChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for TableHeaderChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        TableHeaderChild::Canvas(builder.build())
    }
}
impl From<Cite> for TableHeaderChild {
    fn from(child: Cite) -> Self {
        TableHeaderChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for TableHeaderChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        TableHeaderChild::Cite(builder.build())
    }
}
impl From<Code> for TableHeaderChild {
    fn from(child: Code) -> Self {
        TableHeaderChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for TableHeaderChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        TableHeaderChild::Code(builder.build())
    }
}
impl From<Custom> for TableHeaderChild {
    fn from(child: Custom) -> Self {
        TableHeaderChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for TableHeaderChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        TableHeaderChild::Custom(builder.build())
    }
}
impl From<Data> for TableHeaderChild {
    fn from(child: Data) -> Self {
        TableHeaderChild::Data(child)
    }
}
impl From<builders::DataBuilder> for TableHeaderChild {
    fn from(builder: builders::DataBuilder) -> Self {
        TableHeaderChild::Data(builder.build())
    }
}
impl From<DataList> for TableHeaderChild {
    fn from(child: DataList) -> Self {
        TableHeaderChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for TableHeaderChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        TableHeaderChild::DataList(builder.build())
    }
}
impl From<Definition> for TableHeaderChild {
    fn from(child: Definition) -> Self {
        TableHeaderChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for TableHeaderChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        TableHeaderChild::Definition(builder.build())
    }
}
impl From<Deleted> for TableHeaderChild {
    fn from(child: Deleted) -> Self {
        TableHeaderChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for TableHeaderChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        TableHeaderChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for TableHeaderChild {
    fn from(child: DescriptionList) -> Self {
        TableHeaderChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for TableHeaderChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        TableHeaderChild::DescriptionList(builder.build())
    }
}
impl From<Details> for TableHeaderChild {
    fn from(child: Details) -> Self {
        TableHeaderChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for TableHeaderChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        TableHeaderChild::Details(builder.build())
    }
}
impl From<Dialog> for TableHeaderChild {
    fn from(child: Dialog) -> Self {
        TableHeaderChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for TableHeaderChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        TableHeaderChild::Dialog(builder.build())
    }
}
impl From<Division> for TableHeaderChild {
    fn from(child: Division) -> Self {
        TableHeaderChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for TableHeaderChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        TableHeaderChild::Division(builder.build())
    }
}
impl From<Embed> for TableHeaderChild {
    fn from(child: Embed) -> Self {
        TableHeaderChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for TableHeaderChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        TableHeaderChild::Embed(builder.build())
    }
}
impl From<Emphasis> for TableHeaderChild {
    fn from(child: Emphasis) -> Self {
        TableHeaderChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for TableHeaderChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        TableHeaderChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for TableHeaderChild {
    fn from(child: FieldSet) -> Self {
        TableHeaderChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for TableHeaderChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        TableHeaderChild::FieldSet(builder.build())
    }
}
impl From<Figure> for TableHeaderChild {
    fn from(child: Figure) -> Self {
        TableHeaderChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for TableHeaderChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        TableHeaderChild::Figure(builder.build())
    }
}
impl From<Footer> for TableHeaderChild {
    fn from(child: Footer) -> Self {
        TableHeaderChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for TableHeaderChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        TableHeaderChild::Footer(builder.build())
    }
}
impl From<Form> for TableHeaderChild {
    fn from(child: Form) -> Self {
        TableHeaderChild::Form(child)
    }
}
impl From<builders::FormBuilder> for TableHeaderChild {
    fn from(builder: builders::FormBuilder) -> Self {
        TableHeaderChild::Form(builder.build())
    }
}
impl From<Header> for TableHeaderChild {
    fn from(child: Header) -> Self {
        TableHeaderChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for TableHeaderChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        TableHeaderChild::Header(builder.build())
    }
}
impl From<Heading1> for TableHeaderChild {
    fn from(child: Heading1) -> Self {
        TableHeaderChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for TableHeaderChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        TableHeaderChild::Heading1(builder.build())
    }
}
impl From<Heading2> for TableHeaderChild {
    fn from(child: Heading2) -> Self {
        TableHeaderChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for TableHeaderChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        TableHeaderChild::Heading2(builder.build())
    }
}
impl From<Heading3> for TableHeaderChild {
    fn from(child: Heading3) -> Self {
        TableHeaderChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for TableHeaderChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        TableHeaderChild::Heading3(builder.build())
    }
}
impl From<Heading4> for TableHeaderChild {
    fn from(child: Heading4) -> Self {
        TableHeaderChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for TableHeaderChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        TableHeaderChild::Heading4(builder.build())
    }
}
impl From<Heading5> for TableHeaderChild {
    fn from(child: Heading5) -> Self {
        TableHeaderChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for TableHeaderChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        TableHeaderChild::Heading5(builder.build())
    }
}
impl From<Heading6> for TableHeaderChild {
    fn from(child: Heading6) -> Self {
        TableHeaderChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for TableHeaderChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        TableHeaderChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for TableHeaderChild {
    fn from(child: HeadingGroup) -> Self {
        TableHeaderChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for TableHeaderChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        TableHeaderChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for TableHeaderChild {
    fn from(child: HorizontalRule) -> Self {
        TableHeaderChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for TableHeaderChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        TableHeaderChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for TableHeaderChild {
    fn from(child: Image) -> Self {
        TableHeaderChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for TableHeaderChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        TableHeaderChild::Image(builder.build())
    }
}
impl From<InlineFrame> for TableHeaderChild {
    fn from(child: InlineFrame) -> Self {
        TableHeaderChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for TableHeaderChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        TableHeaderChild::InlineFrame(builder.build())
    }
}
impl From<Input> for TableHeaderChild {
    fn from(child: Input) -> Self {
        TableHeaderChild::Input(child)
    }
}
impl From<builders::InputBuilder> for TableHeaderChild {
    fn from(builder: builders::InputBuilder) -> Self {
        TableHeaderChild::Input(builder.build())
    }
}
impl From<Inserted> for TableHeaderChild {
    fn from(child: Inserted) -> Self {
        TableHeaderChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for TableHeaderChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        TableHeaderChild::Inserted(builder.build())
    }
}
impl From<Italic> for TableHeaderChild {
    fn from(child: Italic) -> Self {
        TableHeaderChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for TableHeaderChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        TableHeaderChild::Italic(builder.build())
    }
}
impl From<Keyboard> for TableHeaderChild {
    fn from(child: Keyboard) -> Self {
        TableHeaderChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for TableHeaderChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        TableHeaderChild::Keyboard(builder.build())
    }
}
impl From<Label> for TableHeaderChild {
    fn from(child: Label) -> Self {
        TableHeaderChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for TableHeaderChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        TableHeaderChild::Label(builder.build())
    }
}
impl From<LineBreak> for TableHeaderChild {
    fn from(child: LineBreak) -> Self {
        TableHeaderChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for TableHeaderChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        TableHeaderChild::LineBreak(builder.build())
    }
}
impl From<Link> for TableHeaderChild {
    fn from(child: Link) -> Self {
        TableHeaderChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for TableHeaderChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        TableHeaderChild::Link(builder.build())
    }
}
impl From<Main> for TableHeaderChild {
    fn from(child: Main) -> Self {
        TableHeaderChild::Main(child)
    }
}
impl From<builders::MainBuilder> for TableHeaderChild {
    fn from(builder: builders::MainBuilder) -> Self {
        TableHeaderChild::Main(builder.build())
    }
}
impl From<Map> for TableHeaderChild {
    fn from(child: Map) -> Self {
        TableHeaderChild::Map(child)
    }
}
impl From<builders::MapBuilder> for TableHeaderChild {
    fn from(builder: builders::MapBuilder) -> Self {
        TableHeaderChild::Map(builder.build())
    }
}
impl From<MapArea> for TableHeaderChild {
    fn from(child: MapArea) -> Self {
        TableHeaderChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for TableHeaderChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        TableHeaderChild::MapArea(builder.build())
    }
}
impl From<Mark> for TableHeaderChild {
    fn from(child: Mark) -> Self {
        TableHeaderChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for TableHeaderChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        TableHeaderChild::Mark(builder.build())
    }
}
impl From<Menu> for TableHeaderChild {
    fn from(child: Menu) -> Self {
        TableHeaderChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for TableHeaderChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        TableHeaderChild::Menu(builder.build())
    }
}
impl From<Metadata> for TableHeaderChild {
    fn from(child: Metadata) -> Self {
        TableHeaderChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for TableHeaderChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        TableHeaderChild::Metadata(builder.build())
    }
}
impl From<Meter> for TableHeaderChild {
    fn from(child: Meter) -> Self {
        TableHeaderChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for TableHeaderChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        TableHeaderChild::Meter(builder.build())
    }
}
impl From<Navigation> for TableHeaderChild {
    fn from(child: Navigation) -> Self {
        TableHeaderChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for TableHeaderChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        TableHeaderChild::Navigation(builder.build())
    }
}
impl From<NoScript> for TableHeaderChild {
    fn from(child: NoScript) -> Self {
        TableHeaderChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for TableHeaderChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        TableHeaderChild::NoScript(builder.build())
    }
}
impl From<Object> for TableHeaderChild {
    fn from(child: Object) -> Self {
        TableHeaderChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for TableHeaderChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        TableHeaderChild::Object(builder.build())
    }
}
impl From<OrderedList> for TableHeaderChild {
    fn from(child: OrderedList) -> Self {
        TableHeaderChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for TableHeaderChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        TableHeaderChild::OrderedList(builder.build())
    }
}
impl From<Output> for TableHeaderChild {
    fn from(child: Output) -> Self {
        TableHeaderChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for TableHeaderChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        TableHeaderChild::Output(builder.build())
    }
}
impl From<Paragraph> for TableHeaderChild {
    fn from(child: Paragraph) -> Self {
        TableHeaderChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for TableHeaderChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        TableHeaderChild::Paragraph(builder.build())
    }
}
impl From<Picture> for TableHeaderChild {
    fn from(child: Picture) -> Self {
        TableHeaderChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for TableHeaderChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        TableHeaderChild::Picture(builder.build())
    }
}
impl From<Preformatted> for TableHeaderChild {
    fn from(child: Preformatted) -> Self {
        TableHeaderChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for TableHeaderChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        TableHeaderChild::Preformatted(builder.build())
    }
}
impl From<Progress> for TableHeaderChild {
    fn from(child: Progress) -> Self {
        TableHeaderChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for TableHeaderChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        TableHeaderChild::Progress(builder.build())
    }
}
impl From<Quote> for TableHeaderChild {
    fn from(child: Quote) -> Self {
        TableHeaderChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for TableHeaderChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        TableHeaderChild::Quote(builder.build())
    }
}
impl From<Ruby> for TableHeaderChild {
    fn from(child: Ruby) -> Self {
        TableHeaderChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for TableHeaderChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        TableHeaderChild::Ruby(builder.build())
    }
}
impl From<Sample> for TableHeaderChild {
    fn from(child: Sample) -> Self {
        TableHeaderChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for TableHeaderChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        TableHeaderChild::Sample(builder.build())
    }
}
impl From<Script> for TableHeaderChild {
    fn from(child: Script) -> Self {
        TableHeaderChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableHeaderChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableHeaderChild::Script(builder.build())
    }
}
impl From<Search> for TableHeaderChild {
    fn from(child: Search) -> Self {
        TableHeaderChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for TableHeaderChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        TableHeaderChild::Search(builder.build())
    }
}
impl From<Section> for TableHeaderChild {
    fn from(child: Section) -> Self {
        TableHeaderChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for TableHeaderChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        TableHeaderChild::Section(builder.build())
    }
}
impl From<Select> for TableHeaderChild {
    fn from(child: Select) -> Self {
        TableHeaderChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for TableHeaderChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        TableHeaderChild::Select(builder.build())
    }
}
impl From<Slot> for TableHeaderChild {
    fn from(child: Slot) -> Self {
        TableHeaderChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for TableHeaderChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        TableHeaderChild::Slot(builder.build())
    }
}
impl From<Small> for TableHeaderChild {
    fn from(child: Small) -> Self {
        TableHeaderChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for TableHeaderChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        TableHeaderChild::Small(builder.build())
    }
}
impl From<Span> for TableHeaderChild {
    fn from(child: Span) -> Self {
        TableHeaderChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for TableHeaderChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        TableHeaderChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for TableHeaderChild {
    fn from(child: StrikeThrough) -> Self {
        TableHeaderChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for TableHeaderChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        TableHeaderChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for TableHeaderChild {
    fn from(child: Strong) -> Self {
        TableHeaderChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for TableHeaderChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        TableHeaderChild::Strong(builder.build())
    }
}
impl From<SubScript> for TableHeaderChild {
    fn from(child: SubScript) -> Self {
        TableHeaderChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for TableHeaderChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        TableHeaderChild::SubScript(builder.build())
    }
}
impl From<SupScript> for TableHeaderChild {
    fn from(child: SupScript) -> Self {
        TableHeaderChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for TableHeaderChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        TableHeaderChild::SupScript(builder.build())
    }
}
impl From<Table> for TableHeaderChild {
    fn from(child: Table) -> Self {
        TableHeaderChild::Table(child)
    }
}
impl From<builders::TableBuilder> for TableHeaderChild {
    fn from(builder: builders::TableBuilder) -> Self {
        TableHeaderChild::Table(builder.build())
    }
}
impl From<Template> for TableHeaderChild {
    fn from(child: Template) -> Self {
        TableHeaderChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableHeaderChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableHeaderChild::Template(builder.build())
    }
}
impl From<TextArea> for TableHeaderChild {
    fn from(child: TextArea) -> Self {
        TableHeaderChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for TableHeaderChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        TableHeaderChild::TextArea(builder.build())
    }
}
impl From<Time> for TableHeaderChild {
    fn from(child: Time) -> Self {
        TableHeaderChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for TableHeaderChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        TableHeaderChild::Time(builder.build())
    }
}
impl From<Underline> for TableHeaderChild {
    fn from(child: Underline) -> Self {
        TableHeaderChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for TableHeaderChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        TableHeaderChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for TableHeaderChild {
    fn from(child: UnorderedList) -> Self {
        TableHeaderChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for TableHeaderChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        TableHeaderChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for TableHeaderChild {
    fn from(child: Variable) -> Self {
        TableHeaderChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for TableHeaderChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        TableHeaderChild::Variable(builder.build())
    }
}
impl From<Video> for TableHeaderChild {
    fn from(child: Video) -> Self {
        TableHeaderChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for TableHeaderChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        TableHeaderChild::Video(builder.build())
    }
}
impl From<WordBreak> for TableHeaderChild {
    fn from(child: WordBreak) -> Self {
        TableHeaderChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for TableHeaderChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        TableHeaderChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for TableHeaderChild {
    fn from(s: &'static str) -> Self {
        TableHeaderChild::Text(s.into())
    }
}
impl From<String> for TableHeaderChild {
    fn from(s: String) -> Self {
        TableHeaderChild::Text(s.into())
    }
}
impl From<CowStr> for TableHeaderChild {
    fn from(s: CowStr) -> Self {
        TableHeaderChild::Text(s)
    }
}
impl std::fmt::Debug for TableHeaderChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableHeaderChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Address(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Article(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            TableHeaderChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            TableHeaderChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Button(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Code(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Data(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Details(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Division(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Form(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Header(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Image(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Input(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Label(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Link(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Main(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Map(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Object(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Output(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Search(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Section(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Select(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Small(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Span(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Table(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Template(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Time(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Video(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            TableHeaderChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for TableHeaderChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableHeaderChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Address(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Article(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Aside(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Audio(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            TableHeaderChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            TableHeaderChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Bold(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Button(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Cite(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Code(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Custom(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Data(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::DataList(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Definition(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Details(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Division(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Embed(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Figure(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Footer(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Form(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Header(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Image(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Input(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Italic(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Label(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Link(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Main(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Map(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Mark(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Menu(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Meter(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Object(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Output(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Picture(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Progress(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Quote(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Sample(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Search(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Section(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Select(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Slot(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Small(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Span(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Strong(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Table(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Template(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Time(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Underline(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Variable(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Video(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            TableHeaderChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
