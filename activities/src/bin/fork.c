asmlinkage __visible void __softirq_entry __do_softirq(void)
{
    /* Get the pending softirq */
	pending = local_softirq_pending();  

    ...

restart:
	/* Reset the pending bitmask before enabling irqs */
	set_softirq_pending(0);

	local_irq_enable();

	h = softirq_vec;

	while ((softirq_bit = ffs(pending))) {
		...
        /* Call action */
		h->action(h);
		
		h++;
		pending >>= softirq_bit;
	}

    /* Check the pending softirqs for another loop
    if pending softirqs left, kernel should handle them
    However, if the allocated time for interrupt context is over,
    kernel handle them by kernel thread(ksoftirqd)*/
	pending = local_softirq_pending();
	if (pending) {
		if (time_before(jiffies, end) && !need_resched() &&
		    --max_restart)
			goto restart;

		wakeup_softirqd();
	}

	account_softirq_exit(current);
	lockdep_softirq_end(in_hardirq);
	softirq_handle_end();
	current_restore_flags(old_flags, PF_MEMALLOC);
}