use noodle_core::Node;

pub(crate) struct LocalNode {
    inner: Box<dyn Node>,
}

impl Node for LocalNode {
    fn discriminator(&self) -> &str {
        self.inner.discriminator()
    }

    fn input_sockets(&self) -> noodle_core::SocketSet {
        self.inner.input_sockets()
    }

    fn output_sockets(&self) -> noodle_core::SocketSet {
        self.inner.output_sockets()
    }

    fn execute(
        &self,
        values: noodle_core::SocketValues,
        mask: noodle_core::OutputMask,
    ) -> Result<noodle_core::SocketValues, noodle_core::NodeExecutionError> {
        self.inner.execute(
            values,
            mask,
        )
    }

    fn changed_since_last_run(&self) -> bool {
        self.inner.changed_since_last_run()
    }
}