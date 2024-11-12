DAY=01
YEAR=2024
N_EXAMPLES=3

create_year_folder:
	# Do this once to set up a folder for that year
	mkdir ${YEAR}

create_day_folder:
	mkdir ${YEAR}/${DAY}
	mkdir ${YEAR}/${DAY}/src

create: create_day_folder
	cp template/Cargo.toml ${YEAR}/${DAY}/Cargo.toml
	cp template/src/util.rs ${YEAR}/${DAY}/src/util.rs
	cp template/src/main.rs ${YEAR}/${DAY}/src/main.rs
	sed -i 's/_dayxx/_${YEAR}_${DAY}/g' ${YEAR}/${DAY}/Cargo.toml

add_empty_example_file:
	for i in $$(seq 1 ${N_EXAMPLES}); do \
		touch ${YEAR}/${DAY}/example$$i.txt; \
	done

add_empty_data_files:
	touch ${YEAR}/${DAY}/data1.txt
	touch ${YEAR}/${DAY}/data2.txt
	touch ${YEAR}/${DAY}/data3.txt
