// Generated by integration tests
use burn::nn::conv::Conv2d;
use burn::nn::conv::Conv2dConfig;
use burn::nn::BatchNorm;
use burn::nn::BatchNormConfig;
use burn::nn::Linear;
use burn::nn::LinearConfig;
use burn::{
    module::Module,
    tensor::{backend::Backend, Tensor},
};


#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv2d1: Conv2d<B>,
    batchnormalization1: BatchNorm<B, 2>,
    linear1: Linear<B>,
    batchnormalization2: BatchNorm<B, 0>,
}

impl<B: Backend> Model<B> {
    pub fn new_with(record: ModelRecord<B>) -> Self {
        let conv2d1 = Conv2dConfig::new([1, 8], [3, 3])
            .with_stride([1, 1])
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init_with(record.conv2d1);
        let batchnormalization1 = BatchNormConfig::new(8)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init_with(record.batchnormalization1);
        let linear1 = LinearConfig::new(288, 10)
            .with_bias(true)
            .init_with(record.linear1);
        let batchnormalization2 = BatchNormConfig::new(10)
            .with_epsilon(0.000009999999747378752f64)
            .with_momentum(0.8999999761581421f64)
            .init_with(record.batchnormalization2);
        Self {
            conv2d1,
            batchnormalization1,
            linear1,
            batchnormalization2,
        }
    }

    pub fn forward(&self, input1: Tensor<B, 4>) -> Tensor<B, 2> {
        let conv2d1_out1 = self.conv2d1.forward(input1);
        let relu1_out1 = burn::tensor::activation::relu(conv2d1_out1);
        let batchnormalization1_out1 = self.batchnormalization1.forward(relu1_out1);
        let flatten1_out1 = batchnormalization1_out1.flatten(1, 3);
        let linear1_out1 = self.linear1.forward(flatten1_out1);
        let batchnormalization2_out1 = self.batchnormalization2.forward(linear1_out1);
        let logsoftmax1_out1 = burn::tensor::activation::log_softmax(batchnormalization2_out1, 1);
        logsoftmax1_out1
    }
}
