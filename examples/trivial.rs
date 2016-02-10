extern crate ocl;

fn main() {
	use ocl::{ProQue, SimpleDims, Buffer};

	// Define a kernel:
	let kernel = r#"
		kernel void multiply(global float* buffer, private float coeff) {
			buffer[get_global_id(0)] *= coeff;
		}
	"#;

	// Create a big ball of OpenCL-ness:
	let ocl_pq = ProQue::builder().src(kernel).build().unwrap();

	// Set our work dimensions / data set size to something arbitrary:
	let dims = SimpleDims::OneDim(500000);

	// Create a 'Buffer' with a built-in vector and initialize it with random 
	// floats between 0.0 and 20.0:
	let mut buffer: Buffer<f32> = Buffer::with_vec_scrambled((0.0, 20.0), &dims, 
		&ocl_pq.queue());

	// Create a kernel with arguments matching those in the kernel:
	let kernel = ocl_pq.create_kernel("multiply", dims.work_size())
		.arg_buf(&buffer)
		.arg_scl(5.0f32);

	// Enqueue kernel:
	kernel.enqueue(None, None);

	// Read results from the device into result_buffer's local vector:
	buffer.fill_vec();

	// Print a result:
	println!("The value at index [{}] is '{}'!", 90007, buffer[90007]);
}