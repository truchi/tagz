// 🤖 This file is generated!

use crate::*;
/// The `<aside>` element's children.
#[derive(Clone)]
pub enum AsideChild {
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
impl From<Abbreviation> for AsideChild {
    fn from(child: Abbreviation) -> Self {
        AsideChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for AsideChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        AsideChild::Abbreviation(builder.build())
    }
}
impl From<Address> for AsideChild {
    fn from(child: Address) -> Self {
        AsideChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for AsideChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        AsideChild::Address(builder.build())
    }
}
impl From<Anchor> for AsideChild {
    fn from(child: Anchor) -> Self {
        AsideChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for AsideChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        AsideChild::Anchor(builder.build())
    }
}
impl From<Article> for AsideChild {
    fn from(child: Article) -> Self {
        AsideChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for AsideChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        AsideChild::Article(builder.build())
    }
}
impl From<Aside> for AsideChild {
    fn from(child: Aside) -> Self {
        AsideChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for AsideChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        AsideChild::Aside(builder.build())
    }
}
impl From<Audio> for AsideChild {
    fn from(child: Audio) -> Self {
        AsideChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for AsideChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        AsideChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for AsideChild {
    fn from(child: BidirectionalIsolate) -> Self {
        AsideChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for AsideChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        AsideChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for AsideChild {
    fn from(child: BidirectionalOverride) -> Self {
        AsideChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for AsideChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        AsideChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for AsideChild {
    fn from(child: BlockQuote) -> Self {
        AsideChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for AsideChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        AsideChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for AsideChild {
    fn from(child: Bold) -> Self {
        AsideChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for AsideChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        AsideChild::Bold(builder.build())
    }
}
impl From<Button> for AsideChild {
    fn from(child: Button) -> Self {
        AsideChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for AsideChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        AsideChild::Button(builder.build())
    }
}
impl From<Canvas> for AsideChild {
    fn from(child: Canvas) -> Self {
        AsideChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for AsideChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        AsideChild::Canvas(builder.build())
    }
}
impl From<Cite> for AsideChild {
    fn from(child: Cite) -> Self {
        AsideChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for AsideChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        AsideChild::Cite(builder.build())
    }
}
impl From<Code> for AsideChild {
    fn from(child: Code) -> Self {
        AsideChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for AsideChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        AsideChild::Code(builder.build())
    }
}
impl From<Custom> for AsideChild {
    fn from(child: Custom) -> Self {
        AsideChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for AsideChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        AsideChild::Custom(builder.build())
    }
}
impl From<Data> for AsideChild {
    fn from(child: Data) -> Self {
        AsideChild::Data(child)
    }
}
impl From<builders::DataBuilder> for AsideChild {
    fn from(builder: builders::DataBuilder) -> Self {
        AsideChild::Data(builder.build())
    }
}
impl From<DataList> for AsideChild {
    fn from(child: DataList) -> Self {
        AsideChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for AsideChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        AsideChild::DataList(builder.build())
    }
}
impl From<Definition> for AsideChild {
    fn from(child: Definition) -> Self {
        AsideChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for AsideChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        AsideChild::Definition(builder.build())
    }
}
impl From<Deleted> for AsideChild {
    fn from(child: Deleted) -> Self {
        AsideChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for AsideChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        AsideChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for AsideChild {
    fn from(child: DescriptionList) -> Self {
        AsideChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for AsideChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        AsideChild::DescriptionList(builder.build())
    }
}
impl From<Details> for AsideChild {
    fn from(child: Details) -> Self {
        AsideChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for AsideChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        AsideChild::Details(builder.build())
    }
}
impl From<Dialog> for AsideChild {
    fn from(child: Dialog) -> Self {
        AsideChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for AsideChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        AsideChild::Dialog(builder.build())
    }
}
impl From<Division> for AsideChild {
    fn from(child: Division) -> Self {
        AsideChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for AsideChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        AsideChild::Division(builder.build())
    }
}
impl From<Embed> for AsideChild {
    fn from(child: Embed) -> Self {
        AsideChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for AsideChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        AsideChild::Embed(builder.build())
    }
}
impl From<Emphasis> for AsideChild {
    fn from(child: Emphasis) -> Self {
        AsideChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for AsideChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        AsideChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for AsideChild {
    fn from(child: FieldSet) -> Self {
        AsideChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for AsideChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        AsideChild::FieldSet(builder.build())
    }
}
impl From<Figure> for AsideChild {
    fn from(child: Figure) -> Self {
        AsideChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for AsideChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        AsideChild::Figure(builder.build())
    }
}
impl From<Footer> for AsideChild {
    fn from(child: Footer) -> Self {
        AsideChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for AsideChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        AsideChild::Footer(builder.build())
    }
}
impl From<Form> for AsideChild {
    fn from(child: Form) -> Self {
        AsideChild::Form(child)
    }
}
impl From<builders::FormBuilder> for AsideChild {
    fn from(builder: builders::FormBuilder) -> Self {
        AsideChild::Form(builder.build())
    }
}
impl From<Header> for AsideChild {
    fn from(child: Header) -> Self {
        AsideChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for AsideChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        AsideChild::Header(builder.build())
    }
}
impl From<Heading1> for AsideChild {
    fn from(child: Heading1) -> Self {
        AsideChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for AsideChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        AsideChild::Heading1(builder.build())
    }
}
impl From<Heading2> for AsideChild {
    fn from(child: Heading2) -> Self {
        AsideChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for AsideChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        AsideChild::Heading2(builder.build())
    }
}
impl From<Heading3> for AsideChild {
    fn from(child: Heading3) -> Self {
        AsideChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for AsideChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        AsideChild::Heading3(builder.build())
    }
}
impl From<Heading4> for AsideChild {
    fn from(child: Heading4) -> Self {
        AsideChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for AsideChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        AsideChild::Heading4(builder.build())
    }
}
impl From<Heading5> for AsideChild {
    fn from(child: Heading5) -> Self {
        AsideChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for AsideChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        AsideChild::Heading5(builder.build())
    }
}
impl From<Heading6> for AsideChild {
    fn from(child: Heading6) -> Self {
        AsideChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for AsideChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        AsideChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for AsideChild {
    fn from(child: HeadingGroup) -> Self {
        AsideChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for AsideChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        AsideChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for AsideChild {
    fn from(child: HorizontalRule) -> Self {
        AsideChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for AsideChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        AsideChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for AsideChild {
    fn from(child: Image) -> Self {
        AsideChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for AsideChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        AsideChild::Image(builder.build())
    }
}
impl From<InlineFrame> for AsideChild {
    fn from(child: InlineFrame) -> Self {
        AsideChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for AsideChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        AsideChild::InlineFrame(builder.build())
    }
}
impl From<Input> for AsideChild {
    fn from(child: Input) -> Self {
        AsideChild::Input(child)
    }
}
impl From<builders::InputBuilder> for AsideChild {
    fn from(builder: builders::InputBuilder) -> Self {
        AsideChild::Input(builder.build())
    }
}
impl From<Inserted> for AsideChild {
    fn from(child: Inserted) -> Self {
        AsideChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for AsideChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        AsideChild::Inserted(builder.build())
    }
}
impl From<Italic> for AsideChild {
    fn from(child: Italic) -> Self {
        AsideChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for AsideChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        AsideChild::Italic(builder.build())
    }
}
impl From<Keyboard> for AsideChild {
    fn from(child: Keyboard) -> Self {
        AsideChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for AsideChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        AsideChild::Keyboard(builder.build())
    }
}
impl From<Label> for AsideChild {
    fn from(child: Label) -> Self {
        AsideChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for AsideChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        AsideChild::Label(builder.build())
    }
}
impl From<LineBreak> for AsideChild {
    fn from(child: LineBreak) -> Self {
        AsideChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for AsideChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        AsideChild::LineBreak(builder.build())
    }
}
impl From<Link> for AsideChild {
    fn from(child: Link) -> Self {
        AsideChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for AsideChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        AsideChild::Link(builder.build())
    }
}
impl From<Main> for AsideChild {
    fn from(child: Main) -> Self {
        AsideChild::Main(child)
    }
}
impl From<builders::MainBuilder> for AsideChild {
    fn from(builder: builders::MainBuilder) -> Self {
        AsideChild::Main(builder.build())
    }
}
impl From<Map> for AsideChild {
    fn from(child: Map) -> Self {
        AsideChild::Map(child)
    }
}
impl From<builders::MapBuilder> for AsideChild {
    fn from(builder: builders::MapBuilder) -> Self {
        AsideChild::Map(builder.build())
    }
}
impl From<MapArea> for AsideChild {
    fn from(child: MapArea) -> Self {
        AsideChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for AsideChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        AsideChild::MapArea(builder.build())
    }
}
impl From<Mark> for AsideChild {
    fn from(child: Mark) -> Self {
        AsideChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for AsideChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        AsideChild::Mark(builder.build())
    }
}
impl From<Menu> for AsideChild {
    fn from(child: Menu) -> Self {
        AsideChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for AsideChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        AsideChild::Menu(builder.build())
    }
}
impl From<Metadata> for AsideChild {
    fn from(child: Metadata) -> Self {
        AsideChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for AsideChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        AsideChild::Metadata(builder.build())
    }
}
impl From<Meter> for AsideChild {
    fn from(child: Meter) -> Self {
        AsideChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for AsideChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        AsideChild::Meter(builder.build())
    }
}
impl From<Navigation> for AsideChild {
    fn from(child: Navigation) -> Self {
        AsideChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for AsideChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        AsideChild::Navigation(builder.build())
    }
}
impl From<NoScript> for AsideChild {
    fn from(child: NoScript) -> Self {
        AsideChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for AsideChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        AsideChild::NoScript(builder.build())
    }
}
impl From<Object> for AsideChild {
    fn from(child: Object) -> Self {
        AsideChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for AsideChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        AsideChild::Object(builder.build())
    }
}
impl From<OrderedList> for AsideChild {
    fn from(child: OrderedList) -> Self {
        AsideChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for AsideChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        AsideChild::OrderedList(builder.build())
    }
}
impl From<Output> for AsideChild {
    fn from(child: Output) -> Self {
        AsideChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for AsideChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        AsideChild::Output(builder.build())
    }
}
impl From<Paragraph> for AsideChild {
    fn from(child: Paragraph) -> Self {
        AsideChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for AsideChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        AsideChild::Paragraph(builder.build())
    }
}
impl From<Picture> for AsideChild {
    fn from(child: Picture) -> Self {
        AsideChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for AsideChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        AsideChild::Picture(builder.build())
    }
}
impl From<Preformatted> for AsideChild {
    fn from(child: Preformatted) -> Self {
        AsideChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for AsideChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        AsideChild::Preformatted(builder.build())
    }
}
impl From<Progress> for AsideChild {
    fn from(child: Progress) -> Self {
        AsideChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for AsideChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        AsideChild::Progress(builder.build())
    }
}
impl From<Quote> for AsideChild {
    fn from(child: Quote) -> Self {
        AsideChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for AsideChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        AsideChild::Quote(builder.build())
    }
}
impl From<Ruby> for AsideChild {
    fn from(child: Ruby) -> Self {
        AsideChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for AsideChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        AsideChild::Ruby(builder.build())
    }
}
impl From<Sample> for AsideChild {
    fn from(child: Sample) -> Self {
        AsideChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for AsideChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        AsideChild::Sample(builder.build())
    }
}
impl From<Script> for AsideChild {
    fn from(child: Script) -> Self {
        AsideChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for AsideChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        AsideChild::Script(builder.build())
    }
}
impl From<Search> for AsideChild {
    fn from(child: Search) -> Self {
        AsideChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for AsideChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        AsideChild::Search(builder.build())
    }
}
impl From<Section> for AsideChild {
    fn from(child: Section) -> Self {
        AsideChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for AsideChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        AsideChild::Section(builder.build())
    }
}
impl From<Select> for AsideChild {
    fn from(child: Select) -> Self {
        AsideChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for AsideChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        AsideChild::Select(builder.build())
    }
}
impl From<Slot> for AsideChild {
    fn from(child: Slot) -> Self {
        AsideChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for AsideChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        AsideChild::Slot(builder.build())
    }
}
impl From<Small> for AsideChild {
    fn from(child: Small) -> Self {
        AsideChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for AsideChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        AsideChild::Small(builder.build())
    }
}
impl From<Span> for AsideChild {
    fn from(child: Span) -> Self {
        AsideChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for AsideChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        AsideChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for AsideChild {
    fn from(child: StrikeThrough) -> Self {
        AsideChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for AsideChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        AsideChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for AsideChild {
    fn from(child: Strong) -> Self {
        AsideChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for AsideChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        AsideChild::Strong(builder.build())
    }
}
impl From<SubScript> for AsideChild {
    fn from(child: SubScript) -> Self {
        AsideChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for AsideChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        AsideChild::SubScript(builder.build())
    }
}
impl From<SupScript> for AsideChild {
    fn from(child: SupScript) -> Self {
        AsideChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for AsideChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        AsideChild::SupScript(builder.build())
    }
}
impl From<Table> for AsideChild {
    fn from(child: Table) -> Self {
        AsideChild::Table(child)
    }
}
impl From<builders::TableBuilder> for AsideChild {
    fn from(builder: builders::TableBuilder) -> Self {
        AsideChild::Table(builder.build())
    }
}
impl From<Template> for AsideChild {
    fn from(child: Template) -> Self {
        AsideChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for AsideChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        AsideChild::Template(builder.build())
    }
}
impl From<TextArea> for AsideChild {
    fn from(child: TextArea) -> Self {
        AsideChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for AsideChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        AsideChild::TextArea(builder.build())
    }
}
impl From<Time> for AsideChild {
    fn from(child: Time) -> Self {
        AsideChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for AsideChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        AsideChild::Time(builder.build())
    }
}
impl From<Underline> for AsideChild {
    fn from(child: Underline) -> Self {
        AsideChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for AsideChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        AsideChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for AsideChild {
    fn from(child: UnorderedList) -> Self {
        AsideChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for AsideChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        AsideChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for AsideChild {
    fn from(child: Variable) -> Self {
        AsideChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for AsideChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        AsideChild::Variable(builder.build())
    }
}
impl From<Video> for AsideChild {
    fn from(child: Video) -> Self {
        AsideChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for AsideChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        AsideChild::Video(builder.build())
    }
}
impl From<WordBreak> for AsideChild {
    fn from(child: WordBreak) -> Self {
        AsideChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for AsideChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        AsideChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for AsideChild {
    fn from(s: &'static str) -> Self {
        AsideChild::Text(s.into())
    }
}
impl From<String> for AsideChild {
    fn from(s: String) -> Self {
        AsideChild::Text(s.into())
    }
}
impl From<CowStr> for AsideChild {
    fn from(s: CowStr) -> Self {
        AsideChild::Text(s)
    }
}
impl std::fmt::Debug for AsideChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsideChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Address(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Article(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Button(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Code(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Data(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Details(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Division(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Form(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Header(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Image(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Input(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Label(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Link(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Main(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Map(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Object(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Output(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Script(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Search(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Section(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Select(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Small(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Span(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Table(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Template(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Time(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Video(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            AsideChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for AsideChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsideChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Address(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Article(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Aside(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Audio(child) => std::fmt::Display::fmt(child, f),
            AsideChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            AsideChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            AsideChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Bold(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Button(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Cite(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Code(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Custom(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Data(child) => std::fmt::Display::fmt(child, f),
            AsideChild::DataList(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Definition(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            AsideChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Details(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Division(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Embed(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            AsideChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Figure(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Footer(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Form(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Header(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            AsideChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            AsideChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Image(child) => std::fmt::Display::fmt(child, f),
            AsideChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Input(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Italic(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Label(child) => std::fmt::Display::fmt(child, f),
            AsideChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Link(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Main(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Map(child) => std::fmt::Display::fmt(child, f),
            AsideChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Mark(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Menu(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Meter(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            AsideChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Object(child) => std::fmt::Display::fmt(child, f),
            AsideChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Output(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Picture(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Progress(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Quote(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Sample(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Script(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Search(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Section(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Select(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Slot(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Small(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Span(child) => std::fmt::Display::fmt(child, f),
            AsideChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Strong(child) => std::fmt::Display::fmt(child, f),
            AsideChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            AsideChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Table(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Template(child) => std::fmt::Display::fmt(child, f),
            AsideChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Time(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Underline(child) => std::fmt::Display::fmt(child, f),
            AsideChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Variable(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Video(child) => std::fmt::Display::fmt(child, f),
            AsideChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            AsideChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
