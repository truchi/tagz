// 🤖 This file is generated!

use crate::*;
/// The `<video>` element's children.
#[derive(Clone)]
pub enum VideoChild {
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
}
impl From<Abbreviation> for VideoChild {
    fn from(child: Abbreviation) -> Self {
        VideoChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for VideoChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        VideoChild::Abbreviation(builder.build())
    }
}
impl From<Address> for VideoChild {
    fn from(child: Address) -> Self {
        VideoChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for VideoChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        VideoChild::Address(builder.build())
    }
}
impl From<Anchor> for VideoChild {
    fn from(child: Anchor) -> Self {
        VideoChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for VideoChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        VideoChild::Anchor(builder.build())
    }
}
impl From<Article> for VideoChild {
    fn from(child: Article) -> Self {
        VideoChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for VideoChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        VideoChild::Article(builder.build())
    }
}
impl From<Aside> for VideoChild {
    fn from(child: Aside) -> Self {
        VideoChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for VideoChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        VideoChild::Aside(builder.build())
    }
}
impl From<Audio> for VideoChild {
    fn from(child: Audio) -> Self {
        VideoChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for VideoChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        VideoChild::Audio(builder.build())
    }
}
impl From<Base> for VideoChild {
    fn from(child: Base) -> Self {
        VideoChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for VideoChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        VideoChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for VideoChild {
    fn from(child: BidirectionalIsolate) -> Self {
        VideoChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for VideoChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        VideoChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for VideoChild {
    fn from(child: BidirectionalOverride) -> Self {
        VideoChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for VideoChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        VideoChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for VideoChild {
    fn from(child: BlockQuote) -> Self {
        VideoChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for VideoChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        VideoChild::BlockQuote(builder.build())
    }
}
impl From<Body> for VideoChild {
    fn from(child: Body) -> Self {
        VideoChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for VideoChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        VideoChild::Body(builder.build())
    }
}
impl From<Bold> for VideoChild {
    fn from(child: Bold) -> Self {
        VideoChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for VideoChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        VideoChild::Bold(builder.build())
    }
}
impl From<Button> for VideoChild {
    fn from(child: Button) -> Self {
        VideoChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for VideoChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        VideoChild::Button(builder.build())
    }
}
impl From<Canvas> for VideoChild {
    fn from(child: Canvas) -> Self {
        VideoChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for VideoChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        VideoChild::Canvas(builder.build())
    }
}
impl From<Caption> for VideoChild {
    fn from(child: Caption) -> Self {
        VideoChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for VideoChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        VideoChild::Caption(builder.build())
    }
}
impl From<Cite> for VideoChild {
    fn from(child: Cite) -> Self {
        VideoChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for VideoChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        VideoChild::Cite(builder.build())
    }
}
impl From<Code> for VideoChild {
    fn from(child: Code) -> Self {
        VideoChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for VideoChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        VideoChild::Code(builder.build())
    }
}
impl From<Custom> for VideoChild {
    fn from(child: Custom) -> Self {
        VideoChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for VideoChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        VideoChild::Custom(builder.build())
    }
}
impl From<Data> for VideoChild {
    fn from(child: Data) -> Self {
        VideoChild::Data(child)
    }
}
impl From<builders::DataBuilder> for VideoChild {
    fn from(builder: builders::DataBuilder) -> Self {
        VideoChild::Data(builder.build())
    }
}
impl From<DataList> for VideoChild {
    fn from(child: DataList) -> Self {
        VideoChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for VideoChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        VideoChild::DataList(builder.build())
    }
}
impl From<Definition> for VideoChild {
    fn from(child: Definition) -> Self {
        VideoChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for VideoChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        VideoChild::Definition(builder.build())
    }
}
impl From<Deleted> for VideoChild {
    fn from(child: Deleted) -> Self {
        VideoChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for VideoChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        VideoChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for VideoChild {
    fn from(child: DescriptionDetails) -> Self {
        VideoChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for VideoChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        VideoChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for VideoChild {
    fn from(child: DescriptionList) -> Self {
        VideoChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for VideoChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        VideoChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for VideoChild {
    fn from(child: DescriptionTerm) -> Self {
        VideoChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for VideoChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        VideoChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for VideoChild {
    fn from(child: Details) -> Self {
        VideoChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for VideoChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        VideoChild::Details(builder.build())
    }
}
impl From<Dialog> for VideoChild {
    fn from(child: Dialog) -> Self {
        VideoChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for VideoChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        VideoChild::Dialog(builder.build())
    }
}
impl From<Division> for VideoChild {
    fn from(child: Division) -> Self {
        VideoChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for VideoChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        VideoChild::Division(builder.build())
    }
}
impl From<Embed> for VideoChild {
    fn from(child: Embed) -> Self {
        VideoChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for VideoChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        VideoChild::Embed(builder.build())
    }
}
impl From<Emphasis> for VideoChild {
    fn from(child: Emphasis) -> Self {
        VideoChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for VideoChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        VideoChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for VideoChild {
    fn from(child: FieldSet) -> Self {
        VideoChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for VideoChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        VideoChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for VideoChild {
    fn from(child: FieldSetLegend) -> Self {
        VideoChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for VideoChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        VideoChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for VideoChild {
    fn from(child: Figure) -> Self {
        VideoChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for VideoChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        VideoChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for VideoChild {
    fn from(child: FigureCaption) -> Self {
        VideoChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for VideoChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        VideoChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for VideoChild {
    fn from(child: Footer) -> Self {
        VideoChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for VideoChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        VideoChild::Footer(builder.build())
    }
}
impl From<Form> for VideoChild {
    fn from(child: Form) -> Self {
        VideoChild::Form(child)
    }
}
impl From<builders::FormBuilder> for VideoChild {
    fn from(builder: builders::FormBuilder) -> Self {
        VideoChild::Form(builder.build())
    }
}
impl From<Head> for VideoChild {
    fn from(child: Head) -> Self {
        VideoChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for VideoChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        VideoChild::Head(builder.build())
    }
}
impl From<Header> for VideoChild {
    fn from(child: Header) -> Self {
        VideoChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for VideoChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        VideoChild::Header(builder.build())
    }
}
impl From<Heading1> for VideoChild {
    fn from(child: Heading1) -> Self {
        VideoChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for VideoChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        VideoChild::Heading1(builder.build())
    }
}
impl From<Heading2> for VideoChild {
    fn from(child: Heading2) -> Self {
        VideoChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for VideoChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        VideoChild::Heading2(builder.build())
    }
}
impl From<Heading3> for VideoChild {
    fn from(child: Heading3) -> Self {
        VideoChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for VideoChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        VideoChild::Heading3(builder.build())
    }
}
impl From<Heading4> for VideoChild {
    fn from(child: Heading4) -> Self {
        VideoChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for VideoChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        VideoChild::Heading4(builder.build())
    }
}
impl From<Heading5> for VideoChild {
    fn from(child: Heading5) -> Self {
        VideoChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for VideoChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        VideoChild::Heading5(builder.build())
    }
}
impl From<Heading6> for VideoChild {
    fn from(child: Heading6) -> Self {
        VideoChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for VideoChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        VideoChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for VideoChild {
    fn from(child: HeadingGroup) -> Self {
        VideoChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for VideoChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        VideoChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for VideoChild {
    fn from(child: HorizontalRule) -> Self {
        VideoChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for VideoChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        VideoChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for VideoChild {
    fn from(child: Html) -> Self {
        VideoChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for VideoChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        VideoChild::Html(builder.build())
    }
}
impl From<Image> for VideoChild {
    fn from(child: Image) -> Self {
        VideoChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for VideoChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        VideoChild::Image(builder.build())
    }
}
impl From<InlineFrame> for VideoChild {
    fn from(child: InlineFrame) -> Self {
        VideoChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for VideoChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        VideoChild::InlineFrame(builder.build())
    }
}
impl From<Input> for VideoChild {
    fn from(child: Input) -> Self {
        VideoChild::Input(child)
    }
}
impl From<builders::InputBuilder> for VideoChild {
    fn from(builder: builders::InputBuilder) -> Self {
        VideoChild::Input(builder.build())
    }
}
impl From<Inserted> for VideoChild {
    fn from(child: Inserted) -> Self {
        VideoChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for VideoChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        VideoChild::Inserted(builder.build())
    }
}
impl From<Italic> for VideoChild {
    fn from(child: Italic) -> Self {
        VideoChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for VideoChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        VideoChild::Italic(builder.build())
    }
}
impl From<Keyboard> for VideoChild {
    fn from(child: Keyboard) -> Self {
        VideoChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for VideoChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        VideoChild::Keyboard(builder.build())
    }
}
impl From<Label> for VideoChild {
    fn from(child: Label) -> Self {
        VideoChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for VideoChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        VideoChild::Label(builder.build())
    }
}
impl From<LineBreak> for VideoChild {
    fn from(child: LineBreak) -> Self {
        VideoChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for VideoChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        VideoChild::LineBreak(builder.build())
    }
}
impl From<Link> for VideoChild {
    fn from(child: Link) -> Self {
        VideoChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for VideoChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        VideoChild::Link(builder.build())
    }
}
impl From<ListItem> for VideoChild {
    fn from(child: ListItem) -> Self {
        VideoChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for VideoChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        VideoChild::ListItem(builder.build())
    }
}
impl From<Main> for VideoChild {
    fn from(child: Main) -> Self {
        VideoChild::Main(child)
    }
}
impl From<builders::MainBuilder> for VideoChild {
    fn from(builder: builders::MainBuilder) -> Self {
        VideoChild::Main(builder.build())
    }
}
impl From<Map> for VideoChild {
    fn from(child: Map) -> Self {
        VideoChild::Map(child)
    }
}
impl From<builders::MapBuilder> for VideoChild {
    fn from(builder: builders::MapBuilder) -> Self {
        VideoChild::Map(builder.build())
    }
}
impl From<MapArea> for VideoChild {
    fn from(child: MapArea) -> Self {
        VideoChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for VideoChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        VideoChild::MapArea(builder.build())
    }
}
impl From<Mark> for VideoChild {
    fn from(child: Mark) -> Self {
        VideoChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for VideoChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        VideoChild::Mark(builder.build())
    }
}
impl From<Menu> for VideoChild {
    fn from(child: Menu) -> Self {
        VideoChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for VideoChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        VideoChild::Menu(builder.build())
    }
}
impl From<Metadata> for VideoChild {
    fn from(child: Metadata) -> Self {
        VideoChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for VideoChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        VideoChild::Metadata(builder.build())
    }
}
impl From<Meter> for VideoChild {
    fn from(child: Meter) -> Self {
        VideoChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for VideoChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        VideoChild::Meter(builder.build())
    }
}
impl From<Navigation> for VideoChild {
    fn from(child: Navigation) -> Self {
        VideoChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for VideoChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        VideoChild::Navigation(builder.build())
    }
}
impl From<NoScript> for VideoChild {
    fn from(child: NoScript) -> Self {
        VideoChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for VideoChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        VideoChild::NoScript(builder.build())
    }
}
impl From<Object> for VideoChild {
    fn from(child: Object) -> Self {
        VideoChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for VideoChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        VideoChild::Object(builder.build())
    }
}
impl From<Option> for VideoChild {
    fn from(child: Option) -> Self {
        VideoChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for VideoChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        VideoChild::Option(builder.build())
    }
}
impl From<OptionGroup> for VideoChild {
    fn from(child: OptionGroup) -> Self {
        VideoChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for VideoChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        VideoChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for VideoChild {
    fn from(child: OrderedList) -> Self {
        VideoChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for VideoChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        VideoChild::OrderedList(builder.build())
    }
}
impl From<Output> for VideoChild {
    fn from(child: Output) -> Self {
        VideoChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for VideoChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        VideoChild::Output(builder.build())
    }
}
impl From<Paragraph> for VideoChild {
    fn from(child: Paragraph) -> Self {
        VideoChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for VideoChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        VideoChild::Paragraph(builder.build())
    }
}
impl From<Picture> for VideoChild {
    fn from(child: Picture) -> Self {
        VideoChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for VideoChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        VideoChild::Picture(builder.build())
    }
}
impl From<Preformatted> for VideoChild {
    fn from(child: Preformatted) -> Self {
        VideoChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for VideoChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        VideoChild::Preformatted(builder.build())
    }
}
impl From<Progress> for VideoChild {
    fn from(child: Progress) -> Self {
        VideoChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for VideoChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        VideoChild::Progress(builder.build())
    }
}
impl From<Quote> for VideoChild {
    fn from(child: Quote) -> Self {
        VideoChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for VideoChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        VideoChild::Quote(builder.build())
    }
}
impl From<Ruby> for VideoChild {
    fn from(child: Ruby) -> Self {
        VideoChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for VideoChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        VideoChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for VideoChild {
    fn from(child: RubyParenthesis) -> Self {
        VideoChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for VideoChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        VideoChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for VideoChild {
    fn from(child: RubyText) -> Self {
        VideoChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for VideoChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        VideoChild::RubyText(builder.build())
    }
}
impl From<Sample> for VideoChild {
    fn from(child: Sample) -> Self {
        VideoChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for VideoChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        VideoChild::Sample(builder.build())
    }
}
impl From<Script> for VideoChild {
    fn from(child: Script) -> Self {
        VideoChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for VideoChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        VideoChild::Script(builder.build())
    }
}
impl From<Search> for VideoChild {
    fn from(child: Search) -> Self {
        VideoChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for VideoChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        VideoChild::Search(builder.build())
    }
}
impl From<Section> for VideoChild {
    fn from(child: Section) -> Self {
        VideoChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for VideoChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        VideoChild::Section(builder.build())
    }
}
impl From<Select> for VideoChild {
    fn from(child: Select) -> Self {
        VideoChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for VideoChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        VideoChild::Select(builder.build())
    }
}
impl From<Slot> for VideoChild {
    fn from(child: Slot) -> Self {
        VideoChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for VideoChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        VideoChild::Slot(builder.build())
    }
}
impl From<Small> for VideoChild {
    fn from(child: Small) -> Self {
        VideoChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for VideoChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        VideoChild::Small(builder.build())
    }
}
impl From<Source> for VideoChild {
    fn from(child: Source) -> Self {
        VideoChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for VideoChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        VideoChild::Source(builder.build())
    }
}
impl From<Span> for VideoChild {
    fn from(child: Span) -> Self {
        VideoChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for VideoChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        VideoChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for VideoChild {
    fn from(child: StrikeThrough) -> Self {
        VideoChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for VideoChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        VideoChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for VideoChild {
    fn from(child: Strong) -> Self {
        VideoChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for VideoChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        VideoChild::Strong(builder.build())
    }
}
impl From<Style> for VideoChild {
    fn from(child: Style) -> Self {
        VideoChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for VideoChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        VideoChild::Style(builder.build())
    }
}
impl From<SubScript> for VideoChild {
    fn from(child: SubScript) -> Self {
        VideoChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for VideoChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        VideoChild::SubScript(builder.build())
    }
}
impl From<Summary> for VideoChild {
    fn from(child: Summary) -> Self {
        VideoChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for VideoChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        VideoChild::Summary(builder.build())
    }
}
impl From<SupScript> for VideoChild {
    fn from(child: SupScript) -> Self {
        VideoChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for VideoChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        VideoChild::SupScript(builder.build())
    }
}
impl From<Table> for VideoChild {
    fn from(child: Table) -> Self {
        VideoChild::Table(child)
    }
}
impl From<builders::TableBuilder> for VideoChild {
    fn from(builder: builders::TableBuilder) -> Self {
        VideoChild::Table(builder.build())
    }
}
impl From<TableBody> for VideoChild {
    fn from(child: TableBody) -> Self {
        VideoChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for VideoChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        VideoChild::TableBody(builder.build())
    }
}
impl From<TableCell> for VideoChild {
    fn from(child: TableCell) -> Self {
        VideoChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for VideoChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        VideoChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for VideoChild {
    fn from(child: TableColumn) -> Self {
        VideoChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for VideoChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        VideoChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for VideoChild {
    fn from(child: TableColumnGroup) -> Self {
        VideoChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for VideoChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        VideoChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for VideoChild {
    fn from(child: TableFoot) -> Self {
        VideoChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for VideoChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        VideoChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for VideoChild {
    fn from(child: TableHead) -> Self {
        VideoChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for VideoChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        VideoChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for VideoChild {
    fn from(child: TableHeader) -> Self {
        VideoChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for VideoChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        VideoChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for VideoChild {
    fn from(child: TableRow) -> Self {
        VideoChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for VideoChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        VideoChild::TableRow(builder.build())
    }
}
impl From<Template> for VideoChild {
    fn from(child: Template) -> Self {
        VideoChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for VideoChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        VideoChild::Template(builder.build())
    }
}
impl From<TextArea> for VideoChild {
    fn from(child: TextArea) -> Self {
        VideoChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for VideoChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        VideoChild::TextArea(builder.build())
    }
}
impl From<Time> for VideoChild {
    fn from(child: Time) -> Self {
        VideoChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for VideoChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        VideoChild::Time(builder.build())
    }
}
impl From<Title> for VideoChild {
    fn from(child: Title) -> Self {
        VideoChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for VideoChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        VideoChild::Title(builder.build())
    }
}
impl From<Track> for VideoChild {
    fn from(child: Track) -> Self {
        VideoChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for VideoChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        VideoChild::Track(builder.build())
    }
}
impl From<Underline> for VideoChild {
    fn from(child: Underline) -> Self {
        VideoChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for VideoChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        VideoChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for VideoChild {
    fn from(child: UnorderedList) -> Self {
        VideoChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for VideoChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        VideoChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for VideoChild {
    fn from(child: Variable) -> Self {
        VideoChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for VideoChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        VideoChild::Variable(builder.build())
    }
}
impl From<Video> for VideoChild {
    fn from(child: Video) -> Self {
        VideoChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for VideoChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        VideoChild::Video(builder.build())
    }
}
impl From<WordBreak> for VideoChild {
    fn from(child: WordBreak) -> Self {
        VideoChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for VideoChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        VideoChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for VideoChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Address(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Article(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Base(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Body(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Button(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Code(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Data(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Details(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Division(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Form(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Head(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Header(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Html(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Image(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Input(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Label(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Link(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Main(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Map(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Object(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Option(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Output(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Script(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Search(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Section(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Select(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Small(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Source(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Span(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Style(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Table(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Template(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Time(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Title(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Track(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::Video(child) => std::fmt::Debug::fmt(child, f),
            VideoChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for VideoChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Address(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Article(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Aside(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Audio(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Base(child) => std::fmt::Display::fmt(child, f),
            VideoChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            VideoChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            VideoChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Body(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Bold(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Button(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Caption(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Cite(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Code(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Custom(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Data(child) => std::fmt::Display::fmt(child, f),
            VideoChild::DataList(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Definition(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            VideoChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            VideoChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            VideoChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Details(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Division(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Embed(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            VideoChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            VideoChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Figure(child) => std::fmt::Display::fmt(child, f),
            VideoChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Footer(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Form(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Head(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Header(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            VideoChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            VideoChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Html(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Image(child) => std::fmt::Display::fmt(child, f),
            VideoChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Input(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Italic(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Label(child) => std::fmt::Display::fmt(child, f),
            VideoChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Link(child) => std::fmt::Display::fmt(child, f),
            VideoChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Main(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Map(child) => std::fmt::Display::fmt(child, f),
            VideoChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Mark(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Menu(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Meter(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            VideoChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Object(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Option(child) => std::fmt::Display::fmt(child, f),
            VideoChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            VideoChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Output(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Picture(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Progress(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Quote(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            VideoChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            VideoChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Sample(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Script(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Search(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Section(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Select(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Slot(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Small(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Source(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Span(child) => std::fmt::Display::fmt(child, f),
            VideoChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Strong(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Style(child) => std::fmt::Display::fmt(child, f),
            VideoChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Summary(child) => std::fmt::Display::fmt(child, f),
            VideoChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Table(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Template(child) => std::fmt::Display::fmt(child, f),
            VideoChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Time(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Title(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Track(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Underline(child) => std::fmt::Display::fmt(child, f),
            VideoChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Variable(child) => std::fmt::Display::fmt(child, f),
            VideoChild::Video(child) => std::fmt::Display::fmt(child, f),
            VideoChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
