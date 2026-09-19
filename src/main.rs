use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(name = "snapmark", version, about = "Local-first bookmarks and snippets")]
struct Cli { #[command(subcommand)] command: Command }
#[derive(Subcommand)]
enum Command { Add { name: String, value: String }, List, Search { query: String }, Get { name: String }, Rm { name: String } }
#[derive(Serialize, Deserialize, Default)] struct Store { entries: Vec<Entry> }
#[derive(Serialize, Deserialize, Clone)] struct Entry { name: String, value: String }
fn path()->PathBuf { dirs::data_local_dir().unwrap_or_else(||PathBuf::from(".")).join("snapmark/store.json") }
fn load()->Store { let p=path(); if !p.exists(){return Store::default()} serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap_or_default() }
fn save(s:&Store){let p=path(); if let Some(d)=p.parent(){fs::create_dir_all(d).unwrap();} fs::write(p,serde_json::to_string_pretty(s).unwrap()).unwrap();}
fn main(){let c=Cli::parse();let mut s=load();match c.command{Command::Add{name,value}=>{if let Some(e)=s.entries.iter_mut().find(|e|e.name==name){e.value=value}else{s.entries.push(Entry{name,value})}save(&s);println!("saved.")},Command::List=>for e in s.entries{println!("{}  {}",e.name,e.value)},Command::Search{query}=>{let q=query.to_lowercase();for e in s.entries{if e.name.to_lowercase().contains(&q)||e.value.to_lowercase().contains(&q){println!("{}  {}",e.name,e.value)}}},Command::Get{name}=>match s.entries.into_iter().find(|e|e.name==name){Some(e)=>println!("{}",e.value),None=>eprintln!("not found: {name}")},Command::Rm{name}=>{s.entries.retain(|e|e.name!=name);save(&s);println!("removed.")}}}
