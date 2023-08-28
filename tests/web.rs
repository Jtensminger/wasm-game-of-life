//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::intern;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::{ Universe, Cell };

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn init_small_capacity() -> Universe {
        Universe::with_capacity(5, 5)
}

/* Checks if Universe is properly constructed by initializing R*C universe of dead cells */
#[wasm_bindgen_test]
pub fn test_with_capacity() {
        /* Let's create a smaller Universe */ 
        let small_universe = init_small_capacity();
        /* grab it's cells */
        let small_cells = small_universe.get_cells();
        /* confirm they're all dead */
        assert_eq!(false, small_cells.contains(&Cell::Alive));
}

#[cfg(test)]
pub fn input_line_structure() -> Universe {
        let mut universe = init_small_capacity();
        /* produces this universe state */ 
        /*
                Dead,    Dead,    Dead,     Dead,    Dead, 
                Dead,    Dead,    Alive,    Dead,    Dead,
                Dead,    Dead,    Alive,    Dead,    Dead,
                Dead,    Dead,    Alive,    Dead,    Dead,
                Dead,    Dead,    Dead,     Dead,    Dead
        */
        universe.set_cells(&[(1,2), (2,2), (3,2)]);
        universe
}

#[cfg(test)]
pub fn expected_line_structure() -> Universe {
        let mut universe = init_small_capacity();
        /* produces this universe state */ 
        /*
                Dead,    Dead,     Dead,     Dead,     Dead,
                Dead,    Dead,     Dead,     Dead,     Dead,
                Dead,    Alive,    Alive,    Alive,    Dead,
                Dead,    Dead,     Dead,     Dead,     Dead,
                Dead,    Dead,     Dead,     Dead,     Dead
        */
        universe.set_cells(&[(2,1), (2,2), (2,3)]);
        universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
        /* Let's create a smaller Universe with a small line to test! */ 
        let mut input_universe = input_line_structure();
        /* This is what our line should look like after one tick in our universe. */
        let expected_universe = expected_line_structure();
        /* Call `tick` and then see if the cells in the `Universe`s are the same */
        /* NEW LIVE NEIGHBORS FN: Input Uni 
                [Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Alive, Dead, Dead,
                Dead, Dead, Alive, Dead, Dead,
                Dead, Dead, Alive, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead]
        */
        input_universe.tick();
        /* NEW LIVE NEIGHBORS FN: Input.tick()
                [Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Alive, Dead, Dead,
                Dead, Dead, Alive, Dead, Dead,
                Dead, Dead, Alive, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead]
        */
        /* NEW LIVE NEIGHBORS FN: Expected Uni
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Alive, Alive, Alive, Dead,
                Dead, Dead, Dead, Dead, Dead,
                Dead, Dead, Dead, Dead, Dead]
        */

        assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

