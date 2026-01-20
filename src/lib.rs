use numpy::{PyArray1 , IntoPyArray};
use pyo3::prelude::* ;

fn main()-> PyResult<()>{
    Python::with_gil(|py|{
        let data = vec![2.0, 3.0, 4.0, 7.0, 9.0];
        let np_array : &PyArray1<f64> = data.into_pyarray(py);
        let np = py.import("numpy")?;
        let mean: f64 = np.getattr("mean")?.call1((np_array,))?.extract()?;

        println!("Favourite number of BHUMIKA is {}.", mean);
        Ok(())
    })
}