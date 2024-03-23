use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {

		let tamanho = 12;
		let mut soma = 0;
		let mut contador = 0;
		let mut linha = 1;
		let mut coluna = 11;
		let mut o: String = String::new();
		let path = Path::new("./data/matriz.json");
		let file = File::open(&path).unwrap();
		let reader = BufReader::new(file);
		let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();
		let mut x_times = 1;
		let mut print_vector: Vec<String> = Vec::new();

		println!("Digite a operação desejada: ");

		print!("Soma S ou Média M => \n");
		stdin().read_line(&mut o).unwrap();

		let o = o.trim();
		
		print_vector.push(std::iter::repeat(" . ").take(12).collect::<String>());
		while linha < (tamanho - 1) {
			x_times = 12 - coluna;
			print_vector.push(std::iter::repeat(" . ").take(coluna).collect::<String>() + &std::iter::repeat(" X ").take(x_times).collect::<String>());

			soma += matriz[linha][coluna];
			contador += 1;
			linha += 1;
			match linha {
				1..=5 => coluna -= 1,
				6=> coluna += 0,
				_ => coluna += 1
			}
			if linha > 1 && linha <= 10{
				let mut aux = coluna + 1;
				while aux <= 11 {
					soma +=  matriz[linha][aux];
					contador += 1;
					aux += 1;
				}
			}
		}
		print_vector.push(std::iter::repeat(" . ").take(12).collect::<String>());
	

		if o == "M" {
			println!("Média: {}", soma / contador);
		} else {
			println!("Soma: {}", soma);
		}
		for i in 0..print_vector.len(){
			println!("{}", print_vector[i]);
		}
		

}
