DAY=15
YEAR=2024
N_EXAMPLES=3

create_year_folder:
	# Do this once to set up a folder for that year
	mkdir ${YEAR}

create_day_folder:
	mkdir ${YEAR}/${DAY}
	mkdir ${YEAR}/${DAY}/src


add_empty_example_files:
	for i in $$(seq 1 ${N_EXAMPLES}); do \
		touch ${YEAR}/${DAY}/example$$i.txt; \
	done

add_empty_data_files:
	touch ${YEAR}/${DAY}/data1.txt
	touch ${YEAR}/${DAY}/data2.txt
	touch ${YEAR}/${DAY}/data3.txt

create: create_day_folder add_empty_example_files add_empty_data_files
	cp template/Cargo.toml ${YEAR}/${DAY}/Cargo.toml
	cp template/src/main.rs ${YEAR}/${DAY}/src/main.rs
	sed -i 's/_dayxx/_${YEAR}_${DAY}/g' ${YEAR}/${DAY}/Cargo.toml
	sed -i 's|../util|../../util|g' ${YEAR}/${DAY}/Cargo.toml

