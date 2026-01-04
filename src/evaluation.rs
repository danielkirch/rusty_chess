use dfdx::losses::{binary_cross_entropy_with_logits_loss, cross_entropy_with_logits_loss};
use dfdx::nn::modules::{Conv2D, Cpu, Flatten2D, Linear, ReLU, Sigmoid};
use dfdx::optim::Adam;

// random model architecture
type Model = (
    (Conv2D<12, 64, 3>, ReLU),
    (Conv2D<64, 64, 3>, ReLU),
    (Conv2D<64, 64, 3>, ReLU),
    (Conv2D<64, 64, 3>, ReLU),
    Flatten2D,
    Linear<4096, 3>, // 8*8*64
);

struct PositionEvaluator {
    model: Model,
    opt: Adam<Model>,
}

impl PositionEvaluator {
    pub fn new() -> Self {
        let dev: Cpu = Default::default();
        let model = dev.build_model(Model);
        let opt = Adam::default();
        PositionEvaluator { model, opt }
    }

    pub fn train(&mut self) {
        let (x, target) = self.generate_batch();

        let mut grads = model.alloc_grads();
        let y = self.model.forward_mut(x.traced(grads));
        let loss = cross_entropy_with_logits_loss(y, target);
        loss.backward();
        self.opt.update(&mut self.model, &grads);
    }

    pub fn load(file: &str) -> Self {
        // TODO: implement
        PositionEvaluator::new()
    }
}
