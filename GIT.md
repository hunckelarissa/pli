# Git / GitHub — как держать pl1c

## Почему репозиторий «раздувался»

| Путь | ~размер | В GitHub? |
|------|---------|-----------|
| `research/` (PDF, zip, книги, pl1gcc) | **~1.2 GB** | **Нет** — локальный архив |
| `target/` (Cargo) | ~60+ MB | **Нет** — собирается |
| артефакты тестов (`*.c` / `*.got` / `*.exe`, `mr*.dat`) | растёт | **Нет** — генерируются |
| `compiler/` + `runtime/` + `tests/*.pli` + `docs/` + tools | **~7–13 MB** | **Да** |

Канон языка — уже в `docs/` (Markdown). Сырые PDF остаются у вас локально в `research/`.

## Первый раз (чистый клон / новый remote)

```powershell
cd f:\proj\pli

# убедиться, что research и target не попадут в индекс
git check-ignore -v research\refs target\debug 2>$null

git init
git add .
git status   # не должно быть research/*.PDF, target/, *.exe, tests/must-run/*.c

git commit -m "Initial import: pl1c L0/L1 slice without research archives"

# создать пустой repo на GitHub, затем:
git branch -M main
git remote add origin https://github.com/<you>/<repo>.git
git push -u origin main
```

Ожидаемый объём первого push: порядка **десятков мегабайт**, не гигабайты.

## Ежедневный цикл

```powershell
# перед коммитом — гейты (опционально, но полезно)
cargo build -p pl1c
powershell -File tools/run_must_run.ps1
powershell -File tools/run_must_parse.ps1
powershell -File tools/run_must_fail.ps1

git status
git add -A
git status   # ещё раз: нет ли .exe / .got / PDF
git commit -m "Short why-focused message"
git push
```

Коммитьте **исходники**: `.rs`, `runtime/**/*.c` / `.h`, тесты `.pli` + `.expected`, `docs/`, `tools/`, `VERSION`, `Cargo.*`.

Не коммитьте: `research/*` (кроме `research/README.md`), `target/`, сгенерированный C/got/exe, файлы `mr*.dat` / `OUT*` в корне.

## Проверка «не засорили ли»

```powershell
# что именно игнорируется
git check-ignore -v research\refs\literature some.pdf target\debug\pl1c.exe

# размер того, что реально в индексе (после add)
git ls-files | ForEach-Object { Get-Item $_ -EA SilentlyContinue } |
  Measure-Object Length -Sum |
  ForEach-Object { '{0:N1} MB in index' -f ($_.Sum/1MB) }
```

Если случайно добавили тяжёлое **до** первого push:

```powershell
git rm -r --cached research
git commit -m "Stop tracking local research archives"
```

Если тяжёлое уже в истории remote — одного `.gitignore` мало: нужен `git filter-repo` / BFG. Лучше не пушить `research/` никогда.

## Два слоя данных

```text
GitHub (лёгкий)          Локально (тяжёлый)
─────────────────        ────────────────────
compiler/ runtime/       research/refs/...PDF
docs/  tests/  tools/    research/pl1gcc-...
MANIFEST LITERATURE      research/книги/
```

После `git clone` на новой машине:

1. `cargo build -p pl1c`
2. (по желанию) скопировать/смонтировать свой `research/` рядом — для чтения PDF; для разработки хватает `docs/`.

## Политика веток (минимум)

- `main` — зелёные гейты (`must-run` / `must-parse` / `must-fail`)
- фичи — короткие ветки `feat/...`, `fix/...`
- не хранить секреты / `.env` (сейчас их нет; не заводите в дерево)

Подробности по продукту: [`README.md`](README.md), [`docs/README.md`](docs/README.md), [`research/README.md`](research/README.md).
