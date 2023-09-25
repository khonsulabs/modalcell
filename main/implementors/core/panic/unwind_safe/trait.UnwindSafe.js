(function() {var implementors = {
"modalcell":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"modalcell/threadsafe/enum.ThreadSafe.html\" title=\"enum modalcell::threadsafe::ThreadSafe\">ThreadSafe</a>",1,["modalcell::threadsafe::ThreadSafe"]],["impl&lt;Behavior&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.SharedMode.html\" title=\"struct modalcell::SharedMode\">SharedMode</a>&lt;Behavior&gt;<span class=\"where fmt-newline\">where\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,</span>",1,["modalcell::SharedMode"]],["impl&lt;'a, Behavior&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.ExclusiveMode.html\" title=\"struct modalcell::ExclusiveMode\">ExclusiveMode</a>&lt;'a, Behavior&gt;<span class=\"where fmt-newline\">where\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::ExclusiveMode"]],["impl&lt;T, Behavior&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.ExclusiveCell.html\" title=\"struct modalcell::ExclusiveCell\">ExclusiveCell</a>&lt;T, Behavior&gt;<span class=\"where fmt-newline\">where\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/core/cell/struct.UnsafeCell.html\" title=\"struct core::cell::UnsafeCell\">UnsafeCell</a>&lt;T&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,</span>",1,["modalcell::ExclusiveCell"]],["impl&lt;T, Behavior&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.SharedCell.html\" title=\"struct modalcell::SharedCell\">SharedCell</a>&lt;T, Behavior&gt;<span class=\"where fmt-newline\">where\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/core/cell/struct.UnsafeCell.html\" title=\"struct core::cell::UnsafeCell\">UnsafeCell</a>&lt;T&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a>,</span>",1,["modalcell::SharedCell"]],["impl&lt;'a, T, Behavior&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.RefMut.html\" title=\"struct modalcell::RefMut\">RefMut</a>&lt;'a, T, Behavior&gt;<span class=\"where fmt-newline\">where\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/core/cell/struct.UnsafeCell.html\" title=\"struct core::cell::UnsafeCell\">UnsafeCell</a>&lt;T&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    &lt;Behavior as <a class=\"trait\" href=\"modalcell/trait.Behavior.html\" title=\"trait modalcell::Behavior\">Behavior</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Behavior.html#associatedtype.Container\" title=\"type modalcell::Behavior::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::RefMut"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.UnwindSafe.html\" title=\"trait core::panic::unwind_safe::UnwindSafe\">UnwindSafe</a> for <a class=\"enum\" href=\"modalcell/enum.SingleThreaded.html\" title=\"enum modalcell::SingleThreaded\">SingleThreaded</a>",1,["modalcell::SingleThreaded"]]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()