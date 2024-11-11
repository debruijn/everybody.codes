DAY=01
YEAR=2024
N_EXAMPLES=1

create_year_folder:
	# Do this once to set up a folder for that year
	mkdir ${YEAR}

create_day_folder:
	mkdir ${YEAR}/${DAY}

create: create_day_folder
	# Copy template script and adjust x to day number
	cp util/aoc_x.py aoc_${YEAR}/aoc_${DAY}.py
	sed -i 's/_x/_${DAY}/g' aoc_${YEAR}/aoc_${DAY}.py
	sed -i 's/day=None/day=${DAY}/g' aoc_${YEAR}/aoc_${DAY}.py
	sed -i 's/year=None/year=${YEAR}/g' aoc_${YEAR}/aoc_${DAY}.py

add_empty_example_file:
	for i in $$(seq 1 ${N_EXAMPLES}); do \
		touch aoc_${YEAR}/aoc_${DAY}_exampledata$$i; \
	done

add_empty_data_file:
	touch aoc_${YEAR}/aoc_${DAY}_data

# TODO: put the above steps in a folder per day
# TODO: think about what to do for new 2023 puzzles concerning examples, currently falling back to using manual files

sync:  # Reminder for what command is to sync a project with the committed Pipfile.lock
	pipenv sync

get_cookie:  # Make sure to install browser-cookie3 as well
	aocd-token > ~/.config/aocd/token

cargo_new_folder:  # Make a new cargo (Rust source) library folder for use within Python, within aoc_${YEAR}
	cd aoc_${YEAR}
	cargo new --lib --edition 2021 aoc_rust_${YEAR}
	# I might replace this with making a template and moving that instead (like in `create`)

maturin_develop:  # Develop the Rust library for YEAR into a Python usable package
	cd aoc_${YEAR}/aoc_rust_${YEAR}
	maturin develop