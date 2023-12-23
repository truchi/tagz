// 🤖 This file is generated!

use crate::*;
/// The `<li>` element's children.
#[derive(Clone)]
pub enum ListItemChild {
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
impl From<Abbreviation> for ListItemChild {
    fn from(child: Abbreviation) -> Self {
        ListItemChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for ListItemChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        ListItemChild::Abbreviation(builder.build())
    }
}
impl From<Address> for ListItemChild {
    fn from(child: Address) -> Self {
        ListItemChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for ListItemChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        ListItemChild::Address(builder.build())
    }
}
impl From<Anchor> for ListItemChild {
    fn from(child: Anchor) -> Self {
        ListItemChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for ListItemChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        ListItemChild::Anchor(builder.build())
    }
}
impl From<Article> for ListItemChild {
    fn from(child: Article) -> Self {
        ListItemChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for ListItemChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        ListItemChild::Article(builder.build())
    }
}
impl From<Aside> for ListItemChild {
    fn from(child: Aside) -> Self {
        ListItemChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for ListItemChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        ListItemChild::Aside(builder.build())
    }
}
impl From<Audio> for ListItemChild {
    fn from(child: Audio) -> Self {
        ListItemChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for ListItemChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        ListItemChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for ListItemChild {
    fn from(child: BidirectionalIsolate) -> Self {
        ListItemChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for ListItemChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        ListItemChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for ListItemChild {
    fn from(child: BidirectionalOverride) -> Self {
        ListItemChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for ListItemChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        ListItemChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for ListItemChild {
    fn from(child: BlockQuote) -> Self {
        ListItemChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for ListItemChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        ListItemChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for ListItemChild {
    fn from(child: Bold) -> Self {
        ListItemChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for ListItemChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        ListItemChild::Bold(builder.build())
    }
}
impl From<Button> for ListItemChild {
    fn from(child: Button) -> Self {
        ListItemChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for ListItemChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        ListItemChild::Button(builder.build())
    }
}
impl From<Canvas> for ListItemChild {
    fn from(child: Canvas) -> Self {
        ListItemChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for ListItemChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        ListItemChild::Canvas(builder.build())
    }
}
impl From<Cite> for ListItemChild {
    fn from(child: Cite) -> Self {
        ListItemChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for ListItemChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        ListItemChild::Cite(builder.build())
    }
}
impl From<Code> for ListItemChild {
    fn from(child: Code) -> Self {
        ListItemChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for ListItemChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        ListItemChild::Code(builder.build())
    }
}
impl From<Custom> for ListItemChild {
    fn from(child: Custom) -> Self {
        ListItemChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for ListItemChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        ListItemChild::Custom(builder.build())
    }
}
impl From<Data> for ListItemChild {
    fn from(child: Data) -> Self {
        ListItemChild::Data(child)
    }
}
impl From<builders::DataBuilder> for ListItemChild {
    fn from(builder: builders::DataBuilder) -> Self {
        ListItemChild::Data(builder.build())
    }
}
impl From<DataList> for ListItemChild {
    fn from(child: DataList) -> Self {
        ListItemChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for ListItemChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        ListItemChild::DataList(builder.build())
    }
}
impl From<Definition> for ListItemChild {
    fn from(child: Definition) -> Self {
        ListItemChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for ListItemChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        ListItemChild::Definition(builder.build())
    }
}
impl From<Deleted> for ListItemChild {
    fn from(child: Deleted) -> Self {
        ListItemChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for ListItemChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        ListItemChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for ListItemChild {
    fn from(child: DescriptionList) -> Self {
        ListItemChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for ListItemChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        ListItemChild::DescriptionList(builder.build())
    }
}
impl From<Details> for ListItemChild {
    fn from(child: Details) -> Self {
        ListItemChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for ListItemChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        ListItemChild::Details(builder.build())
    }
}
impl From<Dialog> for ListItemChild {
    fn from(child: Dialog) -> Self {
        ListItemChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for ListItemChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        ListItemChild::Dialog(builder.build())
    }
}
impl From<Division> for ListItemChild {
    fn from(child: Division) -> Self {
        ListItemChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for ListItemChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        ListItemChild::Division(builder.build())
    }
}
impl From<Embed> for ListItemChild {
    fn from(child: Embed) -> Self {
        ListItemChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for ListItemChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        ListItemChild::Embed(builder.build())
    }
}
impl From<Emphasis> for ListItemChild {
    fn from(child: Emphasis) -> Self {
        ListItemChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for ListItemChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        ListItemChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for ListItemChild {
    fn from(child: FieldSet) -> Self {
        ListItemChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for ListItemChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        ListItemChild::FieldSet(builder.build())
    }
}
impl From<Figure> for ListItemChild {
    fn from(child: Figure) -> Self {
        ListItemChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for ListItemChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        ListItemChild::Figure(builder.build())
    }
}
impl From<Footer> for ListItemChild {
    fn from(child: Footer) -> Self {
        ListItemChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for ListItemChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        ListItemChild::Footer(builder.build())
    }
}
impl From<Form> for ListItemChild {
    fn from(child: Form) -> Self {
        ListItemChild::Form(child)
    }
}
impl From<builders::FormBuilder> for ListItemChild {
    fn from(builder: builders::FormBuilder) -> Self {
        ListItemChild::Form(builder.build())
    }
}
impl From<Header> for ListItemChild {
    fn from(child: Header) -> Self {
        ListItemChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for ListItemChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        ListItemChild::Header(builder.build())
    }
}
impl From<Heading1> for ListItemChild {
    fn from(child: Heading1) -> Self {
        ListItemChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for ListItemChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        ListItemChild::Heading1(builder.build())
    }
}
impl From<Heading2> for ListItemChild {
    fn from(child: Heading2) -> Self {
        ListItemChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for ListItemChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        ListItemChild::Heading2(builder.build())
    }
}
impl From<Heading3> for ListItemChild {
    fn from(child: Heading3) -> Self {
        ListItemChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for ListItemChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        ListItemChild::Heading3(builder.build())
    }
}
impl From<Heading4> for ListItemChild {
    fn from(child: Heading4) -> Self {
        ListItemChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for ListItemChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        ListItemChild::Heading4(builder.build())
    }
}
impl From<Heading5> for ListItemChild {
    fn from(child: Heading5) -> Self {
        ListItemChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for ListItemChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        ListItemChild::Heading5(builder.build())
    }
}
impl From<Heading6> for ListItemChild {
    fn from(child: Heading6) -> Self {
        ListItemChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for ListItemChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        ListItemChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for ListItemChild {
    fn from(child: HeadingGroup) -> Self {
        ListItemChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for ListItemChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        ListItemChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for ListItemChild {
    fn from(child: HorizontalRule) -> Self {
        ListItemChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for ListItemChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        ListItemChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for ListItemChild {
    fn from(child: Image) -> Self {
        ListItemChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for ListItemChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        ListItemChild::Image(builder.build())
    }
}
impl From<InlineFrame> for ListItemChild {
    fn from(child: InlineFrame) -> Self {
        ListItemChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for ListItemChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        ListItemChild::InlineFrame(builder.build())
    }
}
impl From<Input> for ListItemChild {
    fn from(child: Input) -> Self {
        ListItemChild::Input(child)
    }
}
impl From<builders::InputBuilder> for ListItemChild {
    fn from(builder: builders::InputBuilder) -> Self {
        ListItemChild::Input(builder.build())
    }
}
impl From<Inserted> for ListItemChild {
    fn from(child: Inserted) -> Self {
        ListItemChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for ListItemChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        ListItemChild::Inserted(builder.build())
    }
}
impl From<Italic> for ListItemChild {
    fn from(child: Italic) -> Self {
        ListItemChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for ListItemChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        ListItemChild::Italic(builder.build())
    }
}
impl From<Keyboard> for ListItemChild {
    fn from(child: Keyboard) -> Self {
        ListItemChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for ListItemChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        ListItemChild::Keyboard(builder.build())
    }
}
impl From<Label> for ListItemChild {
    fn from(child: Label) -> Self {
        ListItemChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for ListItemChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        ListItemChild::Label(builder.build())
    }
}
impl From<LineBreak> for ListItemChild {
    fn from(child: LineBreak) -> Self {
        ListItemChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for ListItemChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        ListItemChild::LineBreak(builder.build())
    }
}
impl From<Link> for ListItemChild {
    fn from(child: Link) -> Self {
        ListItemChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for ListItemChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        ListItemChild::Link(builder.build())
    }
}
impl From<Main> for ListItemChild {
    fn from(child: Main) -> Self {
        ListItemChild::Main(child)
    }
}
impl From<builders::MainBuilder> for ListItemChild {
    fn from(builder: builders::MainBuilder) -> Self {
        ListItemChild::Main(builder.build())
    }
}
impl From<Map> for ListItemChild {
    fn from(child: Map) -> Self {
        ListItemChild::Map(child)
    }
}
impl From<builders::MapBuilder> for ListItemChild {
    fn from(builder: builders::MapBuilder) -> Self {
        ListItemChild::Map(builder.build())
    }
}
impl From<MapArea> for ListItemChild {
    fn from(child: MapArea) -> Self {
        ListItemChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for ListItemChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        ListItemChild::MapArea(builder.build())
    }
}
impl From<Mark> for ListItemChild {
    fn from(child: Mark) -> Self {
        ListItemChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for ListItemChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        ListItemChild::Mark(builder.build())
    }
}
impl From<Menu> for ListItemChild {
    fn from(child: Menu) -> Self {
        ListItemChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for ListItemChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        ListItemChild::Menu(builder.build())
    }
}
impl From<Metadata> for ListItemChild {
    fn from(child: Metadata) -> Self {
        ListItemChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for ListItemChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        ListItemChild::Metadata(builder.build())
    }
}
impl From<Meter> for ListItemChild {
    fn from(child: Meter) -> Self {
        ListItemChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for ListItemChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        ListItemChild::Meter(builder.build())
    }
}
impl From<Navigation> for ListItemChild {
    fn from(child: Navigation) -> Self {
        ListItemChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for ListItemChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        ListItemChild::Navigation(builder.build())
    }
}
impl From<NoScript> for ListItemChild {
    fn from(child: NoScript) -> Self {
        ListItemChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for ListItemChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        ListItemChild::NoScript(builder.build())
    }
}
impl From<Object> for ListItemChild {
    fn from(child: Object) -> Self {
        ListItemChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for ListItemChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        ListItemChild::Object(builder.build())
    }
}
impl From<OrderedList> for ListItemChild {
    fn from(child: OrderedList) -> Self {
        ListItemChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for ListItemChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        ListItemChild::OrderedList(builder.build())
    }
}
impl From<Output> for ListItemChild {
    fn from(child: Output) -> Self {
        ListItemChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for ListItemChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        ListItemChild::Output(builder.build())
    }
}
impl From<Paragraph> for ListItemChild {
    fn from(child: Paragraph) -> Self {
        ListItemChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for ListItemChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        ListItemChild::Paragraph(builder.build())
    }
}
impl From<Picture> for ListItemChild {
    fn from(child: Picture) -> Self {
        ListItemChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for ListItemChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        ListItemChild::Picture(builder.build())
    }
}
impl From<Preformatted> for ListItemChild {
    fn from(child: Preformatted) -> Self {
        ListItemChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for ListItemChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        ListItemChild::Preformatted(builder.build())
    }
}
impl From<Progress> for ListItemChild {
    fn from(child: Progress) -> Self {
        ListItemChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for ListItemChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        ListItemChild::Progress(builder.build())
    }
}
impl From<Quote> for ListItemChild {
    fn from(child: Quote) -> Self {
        ListItemChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for ListItemChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        ListItemChild::Quote(builder.build())
    }
}
impl From<Ruby> for ListItemChild {
    fn from(child: Ruby) -> Self {
        ListItemChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for ListItemChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        ListItemChild::Ruby(builder.build())
    }
}
impl From<Sample> for ListItemChild {
    fn from(child: Sample) -> Self {
        ListItemChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for ListItemChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        ListItemChild::Sample(builder.build())
    }
}
impl From<Script> for ListItemChild {
    fn from(child: Script) -> Self {
        ListItemChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for ListItemChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        ListItemChild::Script(builder.build())
    }
}
impl From<Search> for ListItemChild {
    fn from(child: Search) -> Self {
        ListItemChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for ListItemChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        ListItemChild::Search(builder.build())
    }
}
impl From<Section> for ListItemChild {
    fn from(child: Section) -> Self {
        ListItemChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for ListItemChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        ListItemChild::Section(builder.build())
    }
}
impl From<Select> for ListItemChild {
    fn from(child: Select) -> Self {
        ListItemChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for ListItemChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        ListItemChild::Select(builder.build())
    }
}
impl From<Slot> for ListItemChild {
    fn from(child: Slot) -> Self {
        ListItemChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for ListItemChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        ListItemChild::Slot(builder.build())
    }
}
impl From<Small> for ListItemChild {
    fn from(child: Small) -> Self {
        ListItemChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for ListItemChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        ListItemChild::Small(builder.build())
    }
}
impl From<Span> for ListItemChild {
    fn from(child: Span) -> Self {
        ListItemChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for ListItemChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        ListItemChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for ListItemChild {
    fn from(child: StrikeThrough) -> Self {
        ListItemChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for ListItemChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        ListItemChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for ListItemChild {
    fn from(child: Strong) -> Self {
        ListItemChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for ListItemChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        ListItemChild::Strong(builder.build())
    }
}
impl From<SubScript> for ListItemChild {
    fn from(child: SubScript) -> Self {
        ListItemChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for ListItemChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        ListItemChild::SubScript(builder.build())
    }
}
impl From<SupScript> for ListItemChild {
    fn from(child: SupScript) -> Self {
        ListItemChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for ListItemChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        ListItemChild::SupScript(builder.build())
    }
}
impl From<Table> for ListItemChild {
    fn from(child: Table) -> Self {
        ListItemChild::Table(child)
    }
}
impl From<builders::TableBuilder> for ListItemChild {
    fn from(builder: builders::TableBuilder) -> Self {
        ListItemChild::Table(builder.build())
    }
}
impl From<Template> for ListItemChild {
    fn from(child: Template) -> Self {
        ListItemChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for ListItemChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        ListItemChild::Template(builder.build())
    }
}
impl From<TextArea> for ListItemChild {
    fn from(child: TextArea) -> Self {
        ListItemChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for ListItemChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        ListItemChild::TextArea(builder.build())
    }
}
impl From<Time> for ListItemChild {
    fn from(child: Time) -> Self {
        ListItemChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for ListItemChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        ListItemChild::Time(builder.build())
    }
}
impl From<Underline> for ListItemChild {
    fn from(child: Underline) -> Self {
        ListItemChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for ListItemChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        ListItemChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for ListItemChild {
    fn from(child: UnorderedList) -> Self {
        ListItemChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for ListItemChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        ListItemChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for ListItemChild {
    fn from(child: Variable) -> Self {
        ListItemChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for ListItemChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        ListItemChild::Variable(builder.build())
    }
}
impl From<Video> for ListItemChild {
    fn from(child: Video) -> Self {
        ListItemChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for ListItemChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        ListItemChild::Video(builder.build())
    }
}
impl From<WordBreak> for ListItemChild {
    fn from(child: WordBreak) -> Self {
        ListItemChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for ListItemChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        ListItemChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for ListItemChild {
    fn from(s: &'static str) -> Self {
        ListItemChild::Text(s.into())
    }
}
impl From<String> for ListItemChild {
    fn from(s: String) -> Self {
        ListItemChild::Text(s.into())
    }
}
impl From<CowStr> for ListItemChild {
    fn from(s: CowStr) -> Self {
        ListItemChild::Text(s)
    }
}
impl std::fmt::Debug for ListItemChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListItemChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Address(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Article(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Button(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Code(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Data(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Details(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Division(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Form(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Header(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Image(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Input(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Label(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Link(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Main(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Map(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Object(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Output(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Script(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Search(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Section(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Select(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Small(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Span(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Table(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Template(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Time(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Video(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            ListItemChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for ListItemChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListItemChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Address(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Article(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Aside(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Audio(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ListItemChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            ListItemChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Bold(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Button(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Cite(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Code(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Custom(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Data(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::DataList(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Definition(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Details(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Division(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Embed(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Figure(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Footer(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Form(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Header(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Image(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Input(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Italic(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Label(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Link(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Main(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Map(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Mark(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Menu(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Meter(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Object(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Output(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Picture(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Progress(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Quote(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Sample(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Script(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Search(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Section(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Select(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Slot(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Small(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Span(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Strong(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Table(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Template(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Time(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Underline(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Variable(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Video(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            ListItemChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
