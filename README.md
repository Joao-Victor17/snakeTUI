# 🐍 SnakeTUI

Jogo da cobrinha rodando direto no terminal, escrito em **Rust**.

Projeto de estudo com o objetivo de aprender Rust na prática: gerenciamento de memória, concorrência com threads, e como estruturar um "mini motor de jogo" sem orientação a objetos clássica.

> A ideia inicial era seguir o código de [kriskw1999/ratatui-snake](https://github.com/kriskw1999/ratatui-snake), mas percebi que estava só copiando uma solução pronta — então resolvi modelar o jogo do zero, do meu jeito, mesmo que isso signifique errar e refatorar mais.

---

## 🎮 Como jogar

```bash
cargo run
```

- **Setas direcionais**: move a cobra (para cima, baixo, esquerda, direita)
- **`Q`** ou **`Esc`**: sai do jogo

A cobra se move sozinha a cada tick do jogo; você só escolhe a direção.

---

## 🧱 Stack

| Crate | Para quê |
|---|---|
| [`ratatui`](https://ratatui.rs/) | Renderizar a interface (o "Canvas" do jogo) direto no terminal |
| [`crossterm`](https://docs.rs/crossterm) | Ler eventos de teclado e controlar o terminal em modo raw |
| [`rand`](https://docs.rs/rand) | Sortear a posição da comida |
| [`color-eyre`](https://docs.rs/color-eyre) | Tratamento de erros e panics mais legível |

Rust edition **2024**.

---

## 🗂️ Estrutura do projeto

```
src/
├── main.rs   # Ponto de entrada: monta a App, threads e o loop principal
├── game.rs   # Regras do jogo: direções, comida, limites do tabuleiro, threads
├── snake.rs  # Modelo da cobra: corpo, movimento, crescimento
└── coord.rs  # Coordenadas no grid e conversão para o sistema de desenho
```

A separação segue uma ideia simples: **cada arquivo cuida de uma responsabilidade**.
`coord` não sabe que existe uma cobra. `snake` não sabe como a tela é desenhada. `game` não sabe o que é teclado. Quem conecta tudo é o `main`.

---

## 🧠 Conceitos de Rust usados no projeto

### 1. Structs em vez de classes

Rust não tem classes como Java ou C#. Em vez disso, usamos **`struct`** para guardar dados e **`impl`** para adicionar comportamento a esses dados. É uma forma de organizar orientação a objetos sem herança:

```rust
pub struct Snake {
    body: VecDeque<Coord>,
    direc: Directions,
    speed_x: i64,
    speed_y: i64,
}

impl Snake {
    pub fn move_snake(&mut self, grow_snake: bool) { ... }
}
```

Aqui, `Snake` é só um conjunto de dados. O bloco `impl` é onde moram os métodos — inclusive o `new()`, que funciona como um "construtor" por convenção (não existe construtor especial em Rust).

Composição é usada no lugar de herança: `Snake` guarda uma `VecDeque<Coord>`, e `Coord` não sabe nada sobre cobra. Cada peça resolve seu próprio problema.

### 2. Ownership e borrowing (`&self` vs `&mut self`)

Toda função que só **lê** dados recebe `&self` (referência imutável). Toda função que **modifica** o struct recebe `&mut self` (referência mutável):

```rust
pub fn getter_head_coord(&self) -> (f64, f64) { ... }      // só lê
pub fn move_snake(&mut self, grow_snake: bool) { ... }      // modifica
```

O compilador de Rust garante, em tempo de compilação, que nunca existam duas referências mutáveis ao mesmo dado ao mesmo tempo — isso elimina uma classe inteira de bugs de concorrência que em outras linguagens só aparecem em produção.

### 3. Enums que carregam significado

`Directions` é um enum simples, mas mostra bem o poder do `match` do Rust: toda vez que uma direção é tratada, o compilador obriga a cobrir todos os casos.

```rust
pub enum Directions { Up, Right, Down, Left }

pub fn is_opposite(&self) -> Self {
    match self {
        Directions::Up => Directions::Down,
        Directions::Down => Directions::Up,
        Directions::Left => Directions::Right,
        Directions::Right => Directions::Left,
    }
}
```

Isso é usado para impedir uma jogada clássica de bug em jogos de cobrinha: virar 180° instantaneamente e "morrer" batendo no próprio corpo.

### 4. Traits: `Default`, `Clone`, `Copy`, `PartialEq`

Várias structs usam `#[derive(...)]` para ganhar comportamento de graça:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coord { x: i64, y: i64 }
```

- `Default` permite `Coord::default()` (zero, zero)
- `Copy`/`Clone` deixam `Coord` ser copiada por valor em vez de "movida" (importante porque coordenadas são pequenas e baratas de copiar)
- `PartialEq` permite comparar duas coordenadas com `==`

Isso é uma forma de **polimorfismo por trait**, diferente de herança: em vez de uma superclasse `Comparable`, cada tipo "opta" em implementar comportamentos específicos.

---

## 🧵 Concorrência: como as threads se encaixam

Esse é o coração técnico do projeto. O jogo roda com **três fluxos de execução** conversando por um canal (`mpsc::channel`):

```
┌─────────────────────┐
│ Thread de input      │──┐
│ (crossterm::event)   │  │
└─────────────────────┘  │
                          ├──► mpsc::Sender<Event> ──► canal ──► main (loop)
┌─────────────────────┐  │
│ Thread de tick        │──┘
│ (dorme 50ms e envia)  │
└─────────────────────┘
```

1. **Thread de input** (`handle_input_events`): fica bloqueada em `crossterm::event::read()`, esperando o usuário apertar uma tecla. Quando isso acontece, envia um `Event::Input` pelo canal.

2. **Thread de tick** (`update_snake_state`): dorme 50ms, acorda, envia `Event::UpdateSnakeState`, dorme de novo. É o "coração" que faz a cobra andar sozinha, independente do teclado.

3. **Thread principal**: não faz nenhum trabalho pesado. Ela só fica em `rx.recv()`, esperando qualquer uma das duas threads acima mandar uma mensagem, e reage:

```rust
match rx.recv().unwrap() {
    Event::Input(key_event) => self.handle_key_event(key_event)?,
    Event::UpdateSnakeState => {
        self.move_or_grow_snake();
        self.check_game_over();
    }
}
```

### Por que esse padrão (e não um `loop` único com `sleep`)?

Se tudo rodasse numa única thread com um `sleep(50ms)`, o teclado só seria lido a cada 50ms — o jogo pareceria "travado" ou com resposta atrasada. Separar input e tick em threads diferentes, unificadas por um canal, é uma versão simples do padrão **produtor-consumidor**: várias fontes produzem eventos, um único consumidor decide o que fazer com cada um, sem *race conditions* porque o `mpsc` (multi-producer, single-consumer) já resolve a exclusão mútua internamente.

### `Arc<AtomicBool>` — sinalizando entre threads sem `Mutex`

Para avisar as threads que o jogo acabou, o projeto usa `Arc<AtomicBool>` em vez de um `Mutex<bool>`:

```rust
game_over: Arc<AtomicBool>,
```

- `Arc` (*Atomic Reference Counted*) permite que o mesmo dado seja compartilhado entre múltiplas threads com segurança, contando quantas "posses" existem e liberando a memória quando a última desaparece.
- `AtomicBool` permite ler e escrever um booleano sem precisar de lock — ideal para uma flag simples de "ligado/desligado" lida por várias threads.

É uma escolha mais leve que um `Mutex` para esse caso específico, e mostra atenção a performance mesmo em um projeto de aprendizado.

---

## 🎨 Ratatui: como o desenho funciona

O [`ratatui`](https://ratatui.rs/) é a biblioteca que transforma o terminal em uma "tela". Aqui ela é usada com o widget `Canvas`, que permite desenhar em coordenadas (x, y) em vez de só texto em grade:

```rust
Canvas::default()
    .block(block)
    .x_bounds(Game::get_bounds_x_to_canvas(inner_area))
    .y_bounds(Game::get_bounds_y_to_canvas(inner_area))
    .marker(Marker::HalfBlock)
    .paint(|ctx| {
        ctx.draw(&Points { coords: &snake_pos, color: Color::Red });
        ctx.draw(&Points { coords: &[self.food.get_coords_to_canvas()], color: Color::Red });
    })
    .render(area, buf);
```

Pontos importantes:

- `Marker::HalfBlock` usa caracteres Unicode que representam "meio bloco", dobrando a resolução vertical percebida no terminal (cada célula de texto vira dois "pixels" verticais).
- O jogo implementa a trait `Widget` para `&App`, que é o jeito idiomático do `ratatui` de dizer "esta struct sabe se desenhar":

```rust
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) { ... }
}
```

- O loop principal só chama `terminal.draw(...)` depois de processar um evento — ou seja, a tela só é redesenhada quando algo muda, não em um loop de renderização constante.

---

## ✅ O que já funciona

- Movimento contínuo da cobra em 4 direções
- Bloqueio de virada de 180° (não morre virando pra trás)
- Comida sorteada dentro dos limites do tabuleiro
- Crescimento da cobra ao comer
- Detecção de saída dos limites do grid
- Input e lógica de jogo em threads separadas, sem travar a UI

## 🚧 O que ainda falta (sendo honesto)

Como é um projeto de aprendizado, tem gente vendo isso agora e cabe eu ser sincero sobre o estado real do código:

- **Sem tela de "Game Over"**: hoje, quando a cobra sai dos limites, a flag `game_over` é ativada, mas o loop principal (`App::run`) não olha pra ela — ele só encerra quando a tecla `q`/`Esc` é apertada. Na prática, o jogo "trava" visualmente ao invés de anunciar o fim.
- **Comida pode nascer em cima da cobra**: `Food::new` sorteia uma posição dentro dos limites do tabuleiro, mas não verifica se essa posição colide com o corpo da cobra.
- **Sem detecção de colisão com o próprio corpo**: a cobra pode atravessar o próprio rabo sem "morrer".
- **Sem pontuação, sem tela inicial, sem reinício** do jogo após o fim.
- Um pequeno resíduo de código: `impl AddAssign<i32> for Coord` existe mas não é usado em lugar nenhum (o movimento real acontece via `Coord::advance`) — provavelmente uma tentativa anterior que ficou pra trás.

Esses pontos são exatamente os próximos passos do roadmap.

---

## 🗺️ Roadmap

- [ ] Implementar tela/estado de "Game Over" de verdade
- [ ] Impedir que a comida nasça sobre o corpo da cobra
- [ ] Detectar colisão da cobra com o próprio corpo
- [ ] Sistema de pontuação
- [ ] Tela de início e opção de reiniciar

---

## 🚀 Rodando localmente

```bash
git clone <url-do-seu-repo>
cd snakeTUI
cargo run
```

Requer apenas o [Rust](https://www.rust-lang.org/tools/install) instalado (edition 2024, então uma toolchain recente).

---

## 📚 Aprendizados principais deste projeto

- Diferença entre `&self` e `&mut self` na prática, não só na teoria
- Como comunicar threads sem `Mutex` usando `mpsc::channel` e `Arc<AtomicBool>`
- Como modelar um domínio (cobra, comida, coordenadas) em Rust sem herança, usando composição de structs
- Uso de `ratatui::Canvas` para desenhar algo além de texto em um terminal
- Que copiar um projeto pronto ensina bem menos do que travar, errar e resolver sozinho
