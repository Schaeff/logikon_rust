use z3::*;

use env_logger;

pub struct Z3Interface<'ctx>
{
    context: &'ctx Context,
    solver: Solver<'ctx>
}

impl<'ctx> Z3Interface<'ctx>
{
    pub fn with_context(ctx: &'ctx Context) -> Z3Interface<'ctx> {
        Z3Interface {
            context: ctx,
            solver: Solver::new(&ctx)
        }
    }

    pub fn test(&self) {
        let _ = env_logger::init();
    //    let cfg = Config::new();
    //    let ctx = Context::new(&cfg);
        let x = self.context.named_int_const("x");
        let y = self.context.named_int_const("y");
        let zero = self.context.from_i64(0);
        let two = self.context.from_i64(2);
        let seven = self.context.from_i64(7);

        let solver = Solver::new(&self.context);
        solver.assert(&x.gt(&y));
        solver.assert(&y.gt(&zero));
        solver.assert(&y.rem(&seven)._eq(&two));
        solver.assert(&x.add(&[&two]).gt(&seven));
        assert!(solver.check());

        let model = solver.get_model();
        let xv = model.eval(&x).unwrap().as_i64().unwrap();
        let yv = model.eval(&y).unwrap().as_i64().unwrap();
        println!("x: {}", xv);
        println!("y: {}", yv);
        assert!(xv > yv);
        assert!(yv % 7 == 2);
        assert!(xv + 2 > 7);
    }

}
