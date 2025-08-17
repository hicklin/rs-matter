use embassy_time::Instant;
use heapless;

use crate::error::{Error, ErrorCode};
use crate::dm::{ClusterId, EndptId};

struct TaskEntry<'a> {
    endpoint_id: EndptId,
    cluster_id: ClusterId,
    // when: Instant,
    func: fn(*const ()), // callback to call
    ctx: *const (),      // object pointer
    _marker: core::marker::PhantomData<&'a ()>,
}

impl<'a> TaskEntry<'a> {
    fn new(endpoint_id: EndptId, cluster_id: ClusterId, func: fn(*const ()), ctx: *const()) -> Self {
        Self {
            endpoint_id,
            func,
            cluster_id,
            ctx,
            _marker: core::marker::PhantomData
        }
    }

    // Call the task
    fn call(&self) {
        (self.func)(self.ctx);
    }
}



struct Scheduler<'a, const N: usize> {
    queue: [Option<TaskEntry<'a>>; N],
}

impl<'a, const N: usize> Scheduler<'a, N> {

    pub fn add_task(&mut self, task: TaskEntry<'a>) -> Result<(), Error>{
        if self.queue.full() {
            return Err(ErrorCode::Busy.into());
        }

        // todo check if a task with the same endpointId and clusterId exist. In so, delete it?

        // todo store task

        Ok(())
    }

    // Stop a task.
    // It is assumed that there will not be mare that one task per endpoint per cluster scheduled.
    pub fn stop_task(&mut self, endpoint: EndptId, cluster: ClusterId) {
        for i in 0..N {
            if let Some(task) = self.queue[i] {
                if task.endpoint_id == endpoint && task.cluster_id == cluster {
                    self.queue[i] = None;
                }
            }
        }
    }
    
    pub async fn run() {
        loop {

        }
    }
}