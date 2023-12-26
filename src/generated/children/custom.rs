// 🤖 This file is generated!

use crate::*;
/// The `<custom>` element's children.
#[derive(Clone)]
pub enum CustomChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    Base(Base),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
    Body(Body),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    DescriptionDetails(DescriptionDetails),
    DescriptionList(DescriptionList),
    DescriptionTerm(DescriptionTerm),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    FieldSetLegend(FieldSetLegend),
    Figure(Figure),
    FigureCaption(FigureCaption),
    Footer(Footer),
    Form(Form),
    Head(Head),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Html(Html),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    ListItem(ListItem),
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
    Option(Option),
    OptionGroup(OptionGroup),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    RubyParenthesis(RubyParenthesis),
    RubyText(RubyText),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    Style(Style),
    SubScript(SubScript),
    Summary(Summary),
    SupScript(SupScript),
    Table(Table),
    TableBody(TableBody),
    TableCell(TableCell),
    TableColumn(TableColumn),
    TableColumnGroup(TableColumnGroup),
    TableFoot(TableFoot),
    TableHead(TableHead),
    TableHeader(TableHeader),
    TableRow(TableRow),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Title(Title),
    Track(Track),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for CustomChild {
    fn from(child: Abbreviation) -> Self {
        CustomChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for CustomChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        CustomChild::Abbreviation(builder.build())
    }
}
impl From<Address> for CustomChild {
    fn from(child: Address) -> Self {
        CustomChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for CustomChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        CustomChild::Address(builder.build())
    }
}
impl From<Anchor> for CustomChild {
    fn from(child: Anchor) -> Self {
        CustomChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for CustomChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        CustomChild::Anchor(builder.build())
    }
}
impl From<Article> for CustomChild {
    fn from(child: Article) -> Self {
        CustomChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for CustomChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        CustomChild::Article(builder.build())
    }
}
impl From<Aside> for CustomChild {
    fn from(child: Aside) -> Self {
        CustomChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for CustomChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        CustomChild::Aside(builder.build())
    }
}
impl From<Audio> for CustomChild {
    fn from(child: Audio) -> Self {
        CustomChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for CustomChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        CustomChild::Audio(builder.build())
    }
}
impl From<Base> for CustomChild {
    fn from(child: Base) -> Self {
        CustomChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for CustomChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        CustomChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for CustomChild {
    fn from(child: BidirectionalIsolate) -> Self {
        CustomChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for CustomChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        CustomChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for CustomChild {
    fn from(child: BidirectionalOverride) -> Self {
        CustomChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for CustomChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        CustomChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for CustomChild {
    fn from(child: BlockQuote) -> Self {
        CustomChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for CustomChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        CustomChild::BlockQuote(builder.build())
    }
}
impl From<Body> for CustomChild {
    fn from(child: Body) -> Self {
        CustomChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for CustomChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        CustomChild::Body(builder.build())
    }
}
impl From<Bold> for CustomChild {
    fn from(child: Bold) -> Self {
        CustomChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for CustomChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        CustomChild::Bold(builder.build())
    }
}
impl From<Button> for CustomChild {
    fn from(child: Button) -> Self {
        CustomChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for CustomChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        CustomChild::Button(builder.build())
    }
}
impl From<Canvas> for CustomChild {
    fn from(child: Canvas) -> Self {
        CustomChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for CustomChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        CustomChild::Canvas(builder.build())
    }
}
impl From<Caption> for CustomChild {
    fn from(child: Caption) -> Self {
        CustomChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for CustomChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        CustomChild::Caption(builder.build())
    }
}
impl From<Cite> for CustomChild {
    fn from(child: Cite) -> Self {
        CustomChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for CustomChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        CustomChild::Cite(builder.build())
    }
}
impl From<Code> for CustomChild {
    fn from(child: Code) -> Self {
        CustomChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for CustomChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        CustomChild::Code(builder.build())
    }
}
impl From<Custom> for CustomChild {
    fn from(child: Custom) -> Self {
        CustomChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for CustomChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        CustomChild::Custom(builder.build())
    }
}
impl From<Data> for CustomChild {
    fn from(child: Data) -> Self {
        CustomChild::Data(child)
    }
}
impl From<builders::DataBuilder> for CustomChild {
    fn from(builder: builders::DataBuilder) -> Self {
        CustomChild::Data(builder.build())
    }
}
impl From<DataList> for CustomChild {
    fn from(child: DataList) -> Self {
        CustomChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for CustomChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        CustomChild::DataList(builder.build())
    }
}
impl From<Definition> for CustomChild {
    fn from(child: Definition) -> Self {
        CustomChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for CustomChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        CustomChild::Definition(builder.build())
    }
}
impl From<Deleted> for CustomChild {
    fn from(child: Deleted) -> Self {
        CustomChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for CustomChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        CustomChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for CustomChild {
    fn from(child: DescriptionDetails) -> Self {
        CustomChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for CustomChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        CustomChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for CustomChild {
    fn from(child: DescriptionList) -> Self {
        CustomChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for CustomChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        CustomChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for CustomChild {
    fn from(child: DescriptionTerm) -> Self {
        CustomChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for CustomChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        CustomChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for CustomChild {
    fn from(child: Details) -> Self {
        CustomChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for CustomChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        CustomChild::Details(builder.build())
    }
}
impl From<Dialog> for CustomChild {
    fn from(child: Dialog) -> Self {
        CustomChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for CustomChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        CustomChild::Dialog(builder.build())
    }
}
impl From<Division> for CustomChild {
    fn from(child: Division) -> Self {
        CustomChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for CustomChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        CustomChild::Division(builder.build())
    }
}
impl From<Embed> for CustomChild {
    fn from(child: Embed) -> Self {
        CustomChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for CustomChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        CustomChild::Embed(builder.build())
    }
}
impl From<Emphasis> for CustomChild {
    fn from(child: Emphasis) -> Self {
        CustomChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for CustomChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        CustomChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for CustomChild {
    fn from(child: FieldSet) -> Self {
        CustomChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for CustomChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        CustomChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for CustomChild {
    fn from(child: FieldSetLegend) -> Self {
        CustomChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for CustomChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        CustomChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for CustomChild {
    fn from(child: Figure) -> Self {
        CustomChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for CustomChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        CustomChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for CustomChild {
    fn from(child: FigureCaption) -> Self {
        CustomChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for CustomChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        CustomChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for CustomChild {
    fn from(child: Footer) -> Self {
        CustomChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for CustomChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        CustomChild::Footer(builder.build())
    }
}
impl From<Form> for CustomChild {
    fn from(child: Form) -> Self {
        CustomChild::Form(child)
    }
}
impl From<builders::FormBuilder> for CustomChild {
    fn from(builder: builders::FormBuilder) -> Self {
        CustomChild::Form(builder.build())
    }
}
impl From<Head> for CustomChild {
    fn from(child: Head) -> Self {
        CustomChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for CustomChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        CustomChild::Head(builder.build())
    }
}
impl From<Header> for CustomChild {
    fn from(child: Header) -> Self {
        CustomChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for CustomChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        CustomChild::Header(builder.build())
    }
}
impl From<Heading1> for CustomChild {
    fn from(child: Heading1) -> Self {
        CustomChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for CustomChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        CustomChild::Heading1(builder.build())
    }
}
impl From<Heading2> for CustomChild {
    fn from(child: Heading2) -> Self {
        CustomChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for CustomChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        CustomChild::Heading2(builder.build())
    }
}
impl From<Heading3> for CustomChild {
    fn from(child: Heading3) -> Self {
        CustomChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for CustomChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        CustomChild::Heading3(builder.build())
    }
}
impl From<Heading4> for CustomChild {
    fn from(child: Heading4) -> Self {
        CustomChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for CustomChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        CustomChild::Heading4(builder.build())
    }
}
impl From<Heading5> for CustomChild {
    fn from(child: Heading5) -> Self {
        CustomChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for CustomChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        CustomChild::Heading5(builder.build())
    }
}
impl From<Heading6> for CustomChild {
    fn from(child: Heading6) -> Self {
        CustomChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for CustomChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        CustomChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for CustomChild {
    fn from(child: HeadingGroup) -> Self {
        CustomChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for CustomChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        CustomChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for CustomChild {
    fn from(child: HorizontalRule) -> Self {
        CustomChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for CustomChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        CustomChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for CustomChild {
    fn from(child: Html) -> Self {
        CustomChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for CustomChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        CustomChild::Html(builder.build())
    }
}
impl From<Image> for CustomChild {
    fn from(child: Image) -> Self {
        CustomChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for CustomChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        CustomChild::Image(builder.build())
    }
}
impl From<InlineFrame> for CustomChild {
    fn from(child: InlineFrame) -> Self {
        CustomChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for CustomChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        CustomChild::InlineFrame(builder.build())
    }
}
impl From<Input> for CustomChild {
    fn from(child: Input) -> Self {
        CustomChild::Input(child)
    }
}
impl From<builders::InputBuilder> for CustomChild {
    fn from(builder: builders::InputBuilder) -> Self {
        CustomChild::Input(builder.build())
    }
}
impl From<Inserted> for CustomChild {
    fn from(child: Inserted) -> Self {
        CustomChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for CustomChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        CustomChild::Inserted(builder.build())
    }
}
impl From<Italic> for CustomChild {
    fn from(child: Italic) -> Self {
        CustomChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for CustomChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        CustomChild::Italic(builder.build())
    }
}
impl From<Keyboard> for CustomChild {
    fn from(child: Keyboard) -> Self {
        CustomChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for CustomChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        CustomChild::Keyboard(builder.build())
    }
}
impl From<Label> for CustomChild {
    fn from(child: Label) -> Self {
        CustomChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for CustomChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        CustomChild::Label(builder.build())
    }
}
impl From<LineBreak> for CustomChild {
    fn from(child: LineBreak) -> Self {
        CustomChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for CustomChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        CustomChild::LineBreak(builder.build())
    }
}
impl From<Link> for CustomChild {
    fn from(child: Link) -> Self {
        CustomChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for CustomChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        CustomChild::Link(builder.build())
    }
}
impl From<ListItem> for CustomChild {
    fn from(child: ListItem) -> Self {
        CustomChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for CustomChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        CustomChild::ListItem(builder.build())
    }
}
impl From<Main> for CustomChild {
    fn from(child: Main) -> Self {
        CustomChild::Main(child)
    }
}
impl From<builders::MainBuilder> for CustomChild {
    fn from(builder: builders::MainBuilder) -> Self {
        CustomChild::Main(builder.build())
    }
}
impl From<Map> for CustomChild {
    fn from(child: Map) -> Self {
        CustomChild::Map(child)
    }
}
impl From<builders::MapBuilder> for CustomChild {
    fn from(builder: builders::MapBuilder) -> Self {
        CustomChild::Map(builder.build())
    }
}
impl From<MapArea> for CustomChild {
    fn from(child: MapArea) -> Self {
        CustomChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for CustomChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        CustomChild::MapArea(builder.build())
    }
}
impl From<Mark> for CustomChild {
    fn from(child: Mark) -> Self {
        CustomChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for CustomChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        CustomChild::Mark(builder.build())
    }
}
impl From<Menu> for CustomChild {
    fn from(child: Menu) -> Self {
        CustomChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for CustomChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        CustomChild::Menu(builder.build())
    }
}
impl From<Metadata> for CustomChild {
    fn from(child: Metadata) -> Self {
        CustomChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for CustomChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        CustomChild::Metadata(builder.build())
    }
}
impl From<Meter> for CustomChild {
    fn from(child: Meter) -> Self {
        CustomChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for CustomChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        CustomChild::Meter(builder.build())
    }
}
impl From<Navigation> for CustomChild {
    fn from(child: Navigation) -> Self {
        CustomChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for CustomChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        CustomChild::Navigation(builder.build())
    }
}
impl From<NoScript> for CustomChild {
    fn from(child: NoScript) -> Self {
        CustomChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for CustomChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        CustomChild::NoScript(builder.build())
    }
}
impl From<Object> for CustomChild {
    fn from(child: Object) -> Self {
        CustomChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for CustomChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        CustomChild::Object(builder.build())
    }
}
impl From<Option> for CustomChild {
    fn from(child: Option) -> Self {
        CustomChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for CustomChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        CustomChild::Option(builder.build())
    }
}
impl From<OptionGroup> for CustomChild {
    fn from(child: OptionGroup) -> Self {
        CustomChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for CustomChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        CustomChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for CustomChild {
    fn from(child: OrderedList) -> Self {
        CustomChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for CustomChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        CustomChild::OrderedList(builder.build())
    }
}
impl From<Output> for CustomChild {
    fn from(child: Output) -> Self {
        CustomChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for CustomChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        CustomChild::Output(builder.build())
    }
}
impl From<Paragraph> for CustomChild {
    fn from(child: Paragraph) -> Self {
        CustomChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for CustomChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        CustomChild::Paragraph(builder.build())
    }
}
impl From<Picture> for CustomChild {
    fn from(child: Picture) -> Self {
        CustomChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for CustomChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        CustomChild::Picture(builder.build())
    }
}
impl From<Preformatted> for CustomChild {
    fn from(child: Preformatted) -> Self {
        CustomChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for CustomChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        CustomChild::Preformatted(builder.build())
    }
}
impl From<Progress> for CustomChild {
    fn from(child: Progress) -> Self {
        CustomChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for CustomChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        CustomChild::Progress(builder.build())
    }
}
impl From<Quote> for CustomChild {
    fn from(child: Quote) -> Self {
        CustomChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for CustomChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        CustomChild::Quote(builder.build())
    }
}
impl From<Ruby> for CustomChild {
    fn from(child: Ruby) -> Self {
        CustomChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for CustomChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        CustomChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for CustomChild {
    fn from(child: RubyParenthesis) -> Self {
        CustomChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for CustomChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        CustomChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for CustomChild {
    fn from(child: RubyText) -> Self {
        CustomChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for CustomChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        CustomChild::RubyText(builder.build())
    }
}
impl From<Sample> for CustomChild {
    fn from(child: Sample) -> Self {
        CustomChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for CustomChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        CustomChild::Sample(builder.build())
    }
}
impl From<Script> for CustomChild {
    fn from(child: Script) -> Self {
        CustomChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for CustomChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        CustomChild::Script(builder.build())
    }
}
impl From<Search> for CustomChild {
    fn from(child: Search) -> Self {
        CustomChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for CustomChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        CustomChild::Search(builder.build())
    }
}
impl From<Section> for CustomChild {
    fn from(child: Section) -> Self {
        CustomChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for CustomChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        CustomChild::Section(builder.build())
    }
}
impl From<Select> for CustomChild {
    fn from(child: Select) -> Self {
        CustomChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for CustomChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        CustomChild::Select(builder.build())
    }
}
impl From<Slot> for CustomChild {
    fn from(child: Slot) -> Self {
        CustomChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for CustomChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        CustomChild::Slot(builder.build())
    }
}
impl From<Small> for CustomChild {
    fn from(child: Small) -> Self {
        CustomChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for CustomChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        CustomChild::Small(builder.build())
    }
}
impl From<Source> for CustomChild {
    fn from(child: Source) -> Self {
        CustomChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for CustomChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        CustomChild::Source(builder.build())
    }
}
impl From<Span> for CustomChild {
    fn from(child: Span) -> Self {
        CustomChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for CustomChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        CustomChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for CustomChild {
    fn from(child: StrikeThrough) -> Self {
        CustomChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for CustomChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        CustomChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for CustomChild {
    fn from(child: Strong) -> Self {
        CustomChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for CustomChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        CustomChild::Strong(builder.build())
    }
}
impl From<Style> for CustomChild {
    fn from(child: Style) -> Self {
        CustomChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for CustomChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        CustomChild::Style(builder.build())
    }
}
impl From<SubScript> for CustomChild {
    fn from(child: SubScript) -> Self {
        CustomChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for CustomChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        CustomChild::SubScript(builder.build())
    }
}
impl From<Summary> for CustomChild {
    fn from(child: Summary) -> Self {
        CustomChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for CustomChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        CustomChild::Summary(builder.build())
    }
}
impl From<SupScript> for CustomChild {
    fn from(child: SupScript) -> Self {
        CustomChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for CustomChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        CustomChild::SupScript(builder.build())
    }
}
impl From<Table> for CustomChild {
    fn from(child: Table) -> Self {
        CustomChild::Table(child)
    }
}
impl From<builders::TableBuilder> for CustomChild {
    fn from(builder: builders::TableBuilder) -> Self {
        CustomChild::Table(builder.build())
    }
}
impl From<TableBody> for CustomChild {
    fn from(child: TableBody) -> Self {
        CustomChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for CustomChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        CustomChild::TableBody(builder.build())
    }
}
impl From<TableCell> for CustomChild {
    fn from(child: TableCell) -> Self {
        CustomChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for CustomChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        CustomChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for CustomChild {
    fn from(child: TableColumn) -> Self {
        CustomChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for CustomChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        CustomChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for CustomChild {
    fn from(child: TableColumnGroup) -> Self {
        CustomChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for CustomChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        CustomChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for CustomChild {
    fn from(child: TableFoot) -> Self {
        CustomChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for CustomChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        CustomChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for CustomChild {
    fn from(child: TableHead) -> Self {
        CustomChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for CustomChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        CustomChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for CustomChild {
    fn from(child: TableHeader) -> Self {
        CustomChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for CustomChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        CustomChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for CustomChild {
    fn from(child: TableRow) -> Self {
        CustomChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for CustomChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        CustomChild::TableRow(builder.build())
    }
}
impl From<Template> for CustomChild {
    fn from(child: Template) -> Self {
        CustomChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for CustomChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        CustomChild::Template(builder.build())
    }
}
impl From<TextArea> for CustomChild {
    fn from(child: TextArea) -> Self {
        CustomChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for CustomChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        CustomChild::TextArea(builder.build())
    }
}
impl From<Time> for CustomChild {
    fn from(child: Time) -> Self {
        CustomChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for CustomChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        CustomChild::Time(builder.build())
    }
}
impl From<Title> for CustomChild {
    fn from(child: Title) -> Self {
        CustomChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for CustomChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        CustomChild::Title(builder.build())
    }
}
impl From<Track> for CustomChild {
    fn from(child: Track) -> Self {
        CustomChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for CustomChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        CustomChild::Track(builder.build())
    }
}
impl From<Underline> for CustomChild {
    fn from(child: Underline) -> Self {
        CustomChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for CustomChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        CustomChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for CustomChild {
    fn from(child: UnorderedList) -> Self {
        CustomChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for CustomChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        CustomChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for CustomChild {
    fn from(child: Variable) -> Self {
        CustomChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for CustomChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        CustomChild::Variable(builder.build())
    }
}
impl From<Video> for CustomChild {
    fn from(child: Video) -> Self {
        CustomChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for CustomChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        CustomChild::Video(builder.build())
    }
}
impl From<WordBreak> for CustomChild {
    fn from(child: WordBreak) -> Self {
        CustomChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for CustomChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        CustomChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for CustomChild {
    fn from(s: &'static str) -> Self {
        CustomChild::Text(s.into())
    }
}
impl From<String> for CustomChild {
    fn from(s: String) -> Self {
        CustomChild::Text(s.into())
    }
}
impl From<CowStr> for CustomChild {
    fn from(s: CowStr) -> Self {
        CustomChild::Text(s)
    }
}
impl std::fmt::Debug for CustomChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Address(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Article(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Base(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Body(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Button(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Code(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Data(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Details(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Division(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Form(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Head(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Header(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Html(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Image(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Input(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Label(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Link(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Main(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Map(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Object(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Option(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Output(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Script(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Search(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Section(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Select(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Small(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Source(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Span(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Style(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Table(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Template(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Time(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Title(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Track(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Video(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            CustomChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for CustomChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Address(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Article(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Aside(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Audio(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Base(child) => std::fmt::Display::fmt(child, f),
            CustomChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            CustomChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            CustomChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Body(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Bold(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Button(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Caption(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Cite(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Code(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Custom(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Data(child) => std::fmt::Display::fmt(child, f),
            CustomChild::DataList(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Definition(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            CustomChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            CustomChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            CustomChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Details(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Division(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Embed(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            CustomChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            CustomChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Figure(child) => std::fmt::Display::fmt(child, f),
            CustomChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Footer(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Form(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Head(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Header(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            CustomChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            CustomChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Html(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Image(child) => std::fmt::Display::fmt(child, f),
            CustomChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Input(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Italic(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Label(child) => std::fmt::Display::fmt(child, f),
            CustomChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Link(child) => std::fmt::Display::fmt(child, f),
            CustomChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Main(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Map(child) => std::fmt::Display::fmt(child, f),
            CustomChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Mark(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Menu(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Meter(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            CustomChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Object(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Option(child) => std::fmt::Display::fmt(child, f),
            CustomChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            CustomChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Output(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Picture(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Progress(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Quote(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            CustomChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            CustomChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Sample(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Script(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Search(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Section(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Select(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Slot(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Small(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Source(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Span(child) => std::fmt::Display::fmt(child, f),
            CustomChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Strong(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Style(child) => std::fmt::Display::fmt(child, f),
            CustomChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Summary(child) => std::fmt::Display::fmt(child, f),
            CustomChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Table(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Template(child) => std::fmt::Display::fmt(child, f),
            CustomChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Time(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Title(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Track(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Underline(child) => std::fmt::Display::fmt(child, f),
            CustomChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Variable(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Video(child) => std::fmt::Display::fmt(child, f),
            CustomChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            CustomChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
