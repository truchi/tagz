// 🤖 This file is generated!

use crate::*;
/// The `<dialog>` element's children.
#[derive(Clone)]
pub enum DialogChild {
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
impl From<Abbreviation> for DialogChild {
    fn from(child: Abbreviation) -> Self {
        DialogChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DialogChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DialogChild::Abbreviation(builder.build())
    }
}
impl From<Address> for DialogChild {
    fn from(child: Address) -> Self {
        DialogChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for DialogChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        DialogChild::Address(builder.build())
    }
}
impl From<Anchor> for DialogChild {
    fn from(child: Anchor) -> Self {
        DialogChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DialogChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DialogChild::Anchor(builder.build())
    }
}
impl From<Article> for DialogChild {
    fn from(child: Article) -> Self {
        DialogChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for DialogChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        DialogChild::Article(builder.build())
    }
}
impl From<Aside> for DialogChild {
    fn from(child: Aside) -> Self {
        DialogChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for DialogChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        DialogChild::Aside(builder.build())
    }
}
impl From<Audio> for DialogChild {
    fn from(child: Audio) -> Self {
        DialogChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DialogChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DialogChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DialogChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DialogChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DialogChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DialogChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DialogChild {
    fn from(child: BidirectionalOverride) -> Self {
        DialogChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DialogChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DialogChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for DialogChild {
    fn from(child: BlockQuote) -> Self {
        DialogChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for DialogChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        DialogChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for DialogChild {
    fn from(child: Bold) -> Self {
        DialogChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DialogChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DialogChild::Bold(builder.build())
    }
}
impl From<Button> for DialogChild {
    fn from(child: Button) -> Self {
        DialogChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DialogChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DialogChild::Button(builder.build())
    }
}
impl From<Canvas> for DialogChild {
    fn from(child: Canvas) -> Self {
        DialogChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DialogChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DialogChild::Canvas(builder.build())
    }
}
impl From<Cite> for DialogChild {
    fn from(child: Cite) -> Self {
        DialogChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DialogChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DialogChild::Cite(builder.build())
    }
}
impl From<Code> for DialogChild {
    fn from(child: Code) -> Self {
        DialogChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DialogChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DialogChild::Code(builder.build())
    }
}
impl From<Custom> for DialogChild {
    fn from(child: Custom) -> Self {
        DialogChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DialogChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DialogChild::Custom(builder.build())
    }
}
impl From<Data> for DialogChild {
    fn from(child: Data) -> Self {
        DialogChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DialogChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DialogChild::Data(builder.build())
    }
}
impl From<DataList> for DialogChild {
    fn from(child: DataList) -> Self {
        DialogChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DialogChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DialogChild::DataList(builder.build())
    }
}
impl From<Definition> for DialogChild {
    fn from(child: Definition) -> Self {
        DialogChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DialogChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DialogChild::Definition(builder.build())
    }
}
impl From<Deleted> for DialogChild {
    fn from(child: Deleted) -> Self {
        DialogChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DialogChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DialogChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for DialogChild {
    fn from(child: DescriptionList) -> Self {
        DialogChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for DialogChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        DialogChild::DescriptionList(builder.build())
    }
}
impl From<Details> for DialogChild {
    fn from(child: Details) -> Self {
        DialogChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for DialogChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        DialogChild::Details(builder.build())
    }
}
impl From<Dialog> for DialogChild {
    fn from(child: Dialog) -> Self {
        DialogChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for DialogChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        DialogChild::Dialog(builder.build())
    }
}
impl From<Division> for DialogChild {
    fn from(child: Division) -> Self {
        DialogChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DialogChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DialogChild::Division(builder.build())
    }
}
impl From<Embed> for DialogChild {
    fn from(child: Embed) -> Self {
        DialogChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DialogChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DialogChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DialogChild {
    fn from(child: Emphasis) -> Self {
        DialogChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DialogChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DialogChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for DialogChild {
    fn from(child: FieldSet) -> Self {
        DialogChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for DialogChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        DialogChild::FieldSet(builder.build())
    }
}
impl From<Figure> for DialogChild {
    fn from(child: Figure) -> Self {
        DialogChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for DialogChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        DialogChild::Figure(builder.build())
    }
}
impl From<Footer> for DialogChild {
    fn from(child: Footer) -> Self {
        DialogChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for DialogChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        DialogChild::Footer(builder.build())
    }
}
impl From<Form> for DialogChild {
    fn from(child: Form) -> Self {
        DialogChild::Form(child)
    }
}
impl From<builders::FormBuilder> for DialogChild {
    fn from(builder: builders::FormBuilder) -> Self {
        DialogChild::Form(builder.build())
    }
}
impl From<Header> for DialogChild {
    fn from(child: Header) -> Self {
        DialogChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for DialogChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        DialogChild::Header(builder.build())
    }
}
impl From<Heading1> for DialogChild {
    fn from(child: Heading1) -> Self {
        DialogChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for DialogChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        DialogChild::Heading1(builder.build())
    }
}
impl From<Heading2> for DialogChild {
    fn from(child: Heading2) -> Self {
        DialogChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for DialogChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        DialogChild::Heading2(builder.build())
    }
}
impl From<Heading3> for DialogChild {
    fn from(child: Heading3) -> Self {
        DialogChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for DialogChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        DialogChild::Heading3(builder.build())
    }
}
impl From<Heading4> for DialogChild {
    fn from(child: Heading4) -> Self {
        DialogChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for DialogChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        DialogChild::Heading4(builder.build())
    }
}
impl From<Heading5> for DialogChild {
    fn from(child: Heading5) -> Self {
        DialogChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for DialogChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        DialogChild::Heading5(builder.build())
    }
}
impl From<Heading6> for DialogChild {
    fn from(child: Heading6) -> Self {
        DialogChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for DialogChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        DialogChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for DialogChild {
    fn from(child: HeadingGroup) -> Self {
        DialogChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for DialogChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        DialogChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for DialogChild {
    fn from(child: HorizontalRule) -> Self {
        DialogChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for DialogChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        DialogChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for DialogChild {
    fn from(child: Image) -> Self {
        DialogChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DialogChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DialogChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DialogChild {
    fn from(child: InlineFrame) -> Self {
        DialogChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DialogChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DialogChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DialogChild {
    fn from(child: Input) -> Self {
        DialogChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DialogChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DialogChild::Input(builder.build())
    }
}
impl From<Inserted> for DialogChild {
    fn from(child: Inserted) -> Self {
        DialogChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DialogChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DialogChild::Inserted(builder.build())
    }
}
impl From<Italic> for DialogChild {
    fn from(child: Italic) -> Self {
        DialogChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DialogChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DialogChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DialogChild {
    fn from(child: Keyboard) -> Self {
        DialogChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DialogChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DialogChild::Keyboard(builder.build())
    }
}
impl From<Label> for DialogChild {
    fn from(child: Label) -> Self {
        DialogChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DialogChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DialogChild::Label(builder.build())
    }
}
impl From<LineBreak> for DialogChild {
    fn from(child: LineBreak) -> Self {
        DialogChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DialogChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DialogChild::LineBreak(builder.build())
    }
}
impl From<Link> for DialogChild {
    fn from(child: Link) -> Self {
        DialogChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DialogChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DialogChild::Link(builder.build())
    }
}
impl From<Main> for DialogChild {
    fn from(child: Main) -> Self {
        DialogChild::Main(child)
    }
}
impl From<builders::MainBuilder> for DialogChild {
    fn from(builder: builders::MainBuilder) -> Self {
        DialogChild::Main(builder.build())
    }
}
impl From<Map> for DialogChild {
    fn from(child: Map) -> Self {
        DialogChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DialogChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DialogChild::Map(builder.build())
    }
}
impl From<MapArea> for DialogChild {
    fn from(child: MapArea) -> Self {
        DialogChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DialogChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DialogChild::MapArea(builder.build())
    }
}
impl From<Mark> for DialogChild {
    fn from(child: Mark) -> Self {
        DialogChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DialogChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DialogChild::Mark(builder.build())
    }
}
impl From<Menu> for DialogChild {
    fn from(child: Menu) -> Self {
        DialogChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for DialogChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        DialogChild::Menu(builder.build())
    }
}
impl From<Metadata> for DialogChild {
    fn from(child: Metadata) -> Self {
        DialogChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DialogChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DialogChild::Metadata(builder.build())
    }
}
impl From<Meter> for DialogChild {
    fn from(child: Meter) -> Self {
        DialogChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DialogChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DialogChild::Meter(builder.build())
    }
}
impl From<Navigation> for DialogChild {
    fn from(child: Navigation) -> Self {
        DialogChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for DialogChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        DialogChild::Navigation(builder.build())
    }
}
impl From<NoScript> for DialogChild {
    fn from(child: NoScript) -> Self {
        DialogChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DialogChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DialogChild::NoScript(builder.build())
    }
}
impl From<Object> for DialogChild {
    fn from(child: Object) -> Self {
        DialogChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DialogChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DialogChild::Object(builder.build())
    }
}
impl From<OrderedList> for DialogChild {
    fn from(child: OrderedList) -> Self {
        DialogChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for DialogChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        DialogChild::OrderedList(builder.build())
    }
}
impl From<Output> for DialogChild {
    fn from(child: Output) -> Self {
        DialogChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DialogChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DialogChild::Output(builder.build())
    }
}
impl From<Paragraph> for DialogChild {
    fn from(child: Paragraph) -> Self {
        DialogChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for DialogChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        DialogChild::Paragraph(builder.build())
    }
}
impl From<Picture> for DialogChild {
    fn from(child: Picture) -> Self {
        DialogChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DialogChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DialogChild::Picture(builder.build())
    }
}
impl From<Preformatted> for DialogChild {
    fn from(child: Preformatted) -> Self {
        DialogChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for DialogChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        DialogChild::Preformatted(builder.build())
    }
}
impl From<Progress> for DialogChild {
    fn from(child: Progress) -> Self {
        DialogChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DialogChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DialogChild::Progress(builder.build())
    }
}
impl From<Quote> for DialogChild {
    fn from(child: Quote) -> Self {
        DialogChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DialogChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DialogChild::Quote(builder.build())
    }
}
impl From<Ruby> for DialogChild {
    fn from(child: Ruby) -> Self {
        DialogChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DialogChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DialogChild::Ruby(builder.build())
    }
}
impl From<Sample> for DialogChild {
    fn from(child: Sample) -> Self {
        DialogChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DialogChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DialogChild::Sample(builder.build())
    }
}
impl From<Script> for DialogChild {
    fn from(child: Script) -> Self {
        DialogChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DialogChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DialogChild::Script(builder.build())
    }
}
impl From<Search> for DialogChild {
    fn from(child: Search) -> Self {
        DialogChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for DialogChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        DialogChild::Search(builder.build())
    }
}
impl From<Section> for DialogChild {
    fn from(child: Section) -> Self {
        DialogChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for DialogChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        DialogChild::Section(builder.build())
    }
}
impl From<Select> for DialogChild {
    fn from(child: Select) -> Self {
        DialogChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DialogChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DialogChild::Select(builder.build())
    }
}
impl From<Slot> for DialogChild {
    fn from(child: Slot) -> Self {
        DialogChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DialogChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DialogChild::Slot(builder.build())
    }
}
impl From<Small> for DialogChild {
    fn from(child: Small) -> Self {
        DialogChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DialogChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DialogChild::Small(builder.build())
    }
}
impl From<Span> for DialogChild {
    fn from(child: Span) -> Self {
        DialogChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DialogChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DialogChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DialogChild {
    fn from(child: StrikeThrough) -> Self {
        DialogChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DialogChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DialogChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DialogChild {
    fn from(child: Strong) -> Self {
        DialogChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DialogChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DialogChild::Strong(builder.build())
    }
}
impl From<SubScript> for DialogChild {
    fn from(child: SubScript) -> Self {
        DialogChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DialogChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DialogChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DialogChild {
    fn from(child: SupScript) -> Self {
        DialogChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DialogChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DialogChild::SupScript(builder.build())
    }
}
impl From<Table> for DialogChild {
    fn from(child: Table) -> Self {
        DialogChild::Table(child)
    }
}
impl From<builders::TableBuilder> for DialogChild {
    fn from(builder: builders::TableBuilder) -> Self {
        DialogChild::Table(builder.build())
    }
}
impl From<Template> for DialogChild {
    fn from(child: Template) -> Self {
        DialogChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DialogChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DialogChild::Template(builder.build())
    }
}
impl From<TextArea> for DialogChild {
    fn from(child: TextArea) -> Self {
        DialogChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DialogChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DialogChild::TextArea(builder.build())
    }
}
impl From<Time> for DialogChild {
    fn from(child: Time) -> Self {
        DialogChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DialogChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DialogChild::Time(builder.build())
    }
}
impl From<Underline> for DialogChild {
    fn from(child: Underline) -> Self {
        DialogChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DialogChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DialogChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for DialogChild {
    fn from(child: UnorderedList) -> Self {
        DialogChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for DialogChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        DialogChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for DialogChild {
    fn from(child: Variable) -> Self {
        DialogChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DialogChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DialogChild::Variable(builder.build())
    }
}
impl From<Video> for DialogChild {
    fn from(child: Video) -> Self {
        DialogChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DialogChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DialogChild::Video(builder.build())
    }
}
impl From<WordBreak> for DialogChild {
    fn from(child: WordBreak) -> Self {
        DialogChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DialogChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DialogChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DialogChild {
    fn from(s: &'static str) -> Self {
        DialogChild::Text(s.into())
    }
}
impl From<String> for DialogChild {
    fn from(s: String) -> Self {
        DialogChild::Text(s.into())
    }
}
impl From<CowStr> for DialogChild {
    fn from(s: CowStr) -> Self {
        DialogChild::Text(s)
    }
}
impl std::fmt::Debug for DialogChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Address(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Article(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Details(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Form(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Header(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Main(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Search(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Section(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Table(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DialogChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DialogChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Address(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Article(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Aside(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DialogChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            DialogChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            DialogChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Button(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Code(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Data(child) => std::fmt::Display::fmt(child, f),
            DialogChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DialogChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Details(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Division(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DialogChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Figure(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Footer(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Form(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Header(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            DialogChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            DialogChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Image(child) => std::fmt::Display::fmt(child, f),
            DialogChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Input(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Label(child) => std::fmt::Display::fmt(child, f),
            DialogChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Link(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Main(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Map(child) => std::fmt::Display::fmt(child, f),
            DialogChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Menu(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            DialogChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Object(child) => std::fmt::Display::fmt(child, f),
            DialogChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Output(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Script(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Search(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Section(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Select(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Small(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Span(child) => std::fmt::Display::fmt(child, f),
            DialogChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DialogChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DialogChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Table(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Template(child) => std::fmt::Display::fmt(child, f),
            DialogChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Time(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DialogChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Video(child) => std::fmt::Display::fmt(child, f),
            DialogChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DialogChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
