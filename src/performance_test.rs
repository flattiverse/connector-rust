

use rand;
use rand::Rng;

use std::thread;

use std::sync::Arc;
use std::sync::RwLock;

use Error;
use TimeSpan;
use StopWatch;
use ManualResetEvent;
use net::BinaryWriter;

pub struct PerformanceTest {
    result:  RwLock<i64>,
    memory:  bool,
    ready:   ManualResetEvent,
    control: Arc<ManualResetEvent>,
}

impl PerformanceTest {
    pub fn new(control: Arc<ManualResetEvent>, time: TimeSpan, memory: bool) -> Arc<PerformanceTest> {
        let test = Arc::new(PerformanceTest {
            result: RwLock::new(0_i64),
            control,
            memory,
            ready: ManualResetEvent::new(false),
        });
        let clone = test.clone();
        thread::spawn(move || {
            let time = time;
            clone.test_sequence(&time).unwrap();
        });
        test
    }

    pub fn test_sequence(&self, time: &TimeSpan) -> Result<(), Error> {
        let mut stop_watch = StopWatch::new();
        let mut last_view  = TimeSpan::new(0);

        let mut last_result = 0_i64;
        let mut planned_phase = 1_000_000_i64;
        let mut result = 0_i64;

        self.ready.set()?;
        self.control.wait_one()?;
        self.ready.reset()?;
        stop_watch.start();

        loop {
            for _ in result..planned_phase {
                result += 1;
            }

            let current_measurement = stop_watch.elapsed();

            if current_measurement.ticks() > time.ticks() || (current_measurement.ticks() - last_view.ticks()) <= 10 {
                break;
            }

            planned_phase = (time.ticks() - current_measurement.ticks())
                          * 15_i64
                          * (result - last_result)
                          / (current_measurement.ticks() - last_view.ticks())
                          / 16_i64
                          + result;
            last_view     = current_measurement;
            last_result   = result;
        }

        stop_watch.stop();
        result      = result * 10_000_000_i64 / stop_watch.ticks();
        *self.result.write()? = result;

        if !self.memory {
            self.ready().set()?;
            return Ok(());
        }

        stop_watch.reset();
        last_view     = TimeSpan::new(0);
        planned_phase = 100_i64;
        last_result   = 0_i64;
        result        = 0_i64;

        let mut rand = rand::thread_rng();
        let mut ram_data = Vec::with_capacity(128);

        for _ in 0..128 {
            let mut vec = Vec::with_capacity(1_048_576);
            {
                let writer = &mut vec as &mut BinaryWriter;
                for _ in 0..(1_048_576/8) {
                    writer.write_i64(rand.gen::<i64>())?
                }
            }
            ram_data.push(vec);
        }

        let mut tmp_data = Box::new([0u8; 262_144]);


        self.ready.set()?;
        self.control.wait_one()?;
        self.ready.reset()?;
        stop_watch.start();


        loop {
            for _ in result..planned_phase {
                result += 1;
            }

            let array = (rand.gen::<u8>() / 2_u8) as usize;
            let from  = (rand.gen::<f32>() * 786_432_f32) as usize;
            let to           = from + 262_144;
            tmp_data.clone_from_slice(&ram_data[array][from..to]);

            let current_measurement = stop_watch.elapsed();

            if current_measurement.ticks() > time.ticks() || current_measurement.ticks() - last_view.ticks() <= 10 {
                break;
            }

            planned_phase = (time.ticks() - current_measurement.ticks())
                          * 15_i64
                          * (result - last_result)
                          / (current_measurement.ticks() - last_view.ticks())
                          / 16_i64
                          + result;
            last_view     = current_measurement;
            last_result   = result;
        }

        stop_watch.stop();
        result = result * 10_000_000_i64 / stop_watch.ticks();
        *self.result.write()? = result;
        self.ready.set()?;
        Ok(())
    }

    pub fn try_result(&self) -> Result<i64, Error> {
        Ok(*self.result.read()?)
    }

    pub fn ready(&self) -> &ManualResetEvent {
        &self.ready
    }

    pub fn control(&self) -> &ManualResetEvent {
        &self.control
    }

    pub fn close(self) -> Result<(), Error> {
        self.ready.close()
    }
}