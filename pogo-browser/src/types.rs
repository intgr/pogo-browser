use pogo_data::schema::MonsterType;
use yew::prelude::*;

struct TypeData {
    color: &'static str,
    label: &'static str,
}

fn get_type_data(typ: &MonsterType) -> &'static TypeData {
    match typ {
        MonsterType::Normal => &TypeData { label: "Normal", color: "#A8A878" },
        MonsterType::Fire => &TypeData { label: "Fire", color: "#F08030" },
        MonsterType::Fighting => &TypeData { label: "Fighting", color: "#C03028" },
        MonsterType::Water => &TypeData { label: "Water", color: "#6890F0" },
        MonsterType::Flying => &TypeData { label: "Flying", color: "#A890F0" },
        MonsterType::Grass => &TypeData { label: "Grass", color: "#78C850" },
        MonsterType::Poison => &TypeData { label: "Poison", color: "#A040A0" },
        MonsterType::Electric => &TypeData { label: "Electric", color: "#F8D030" },
        MonsterType::Ground => &TypeData { label: "Ground", color: "#E0C068" },
        MonsterType::Psychic => &TypeData { label: "Psychic", color: "#F85888" },
        MonsterType::Rock => &TypeData { label: "Rock", color: "#B8A038" },
        MonsterType::Ice => &TypeData { label: "Ice", color: "#98D8D8" },
        MonsterType::Bug => &TypeData { label: "Bug", color: "#A8B820" },
        MonsterType::Dragon => &TypeData { label: "Dragon", color: "#7038F8" },
        MonsterType::Ghost => &TypeData { label: "Ghost", color: "#705898" },
        MonsterType::Dark => &TypeData { label: "Dark", color: "#705848" },
        MonsterType::Steel => &TypeData { label: "Steel", color: "#B8B8D0" },
        MonsterType::Fairy => &TypeData { label: "Fairy", color: "#EE99AC" },
    }
}

// TYPE TAG
#[derive(Properties, PartialEq)]
pub struct TypeTagProps {
    pub typ: MonsterType,
}

#[function_component(TypeTag)]
pub fn render_type_tag(props: &TypeTagProps) -> Html {
    let data = get_type_data(&props.typ);
    html! {
        <span class="tag" style={format!("background: {}; color: #fff", data.color)}>{data.label}</span>
    }
}

// TYPE PAIR
#[derive(Properties, PartialEq)]
pub struct TypePairProps {
    pub typ1: MonsterType,
    pub typ2: Option<MonsterType>,
}

#[function_component(TypePair)]
pub fn render_type_pair(props: &TypePairProps) -> Html {
    html! {
        <span class="tags has-addons">
            <TypeTag typ={props.typ1} />
            { props.typ2.clone().map_or_else(|| html! {}, |t| html! {<TypeTag typ={t} />})}
        </span>
    }
}
