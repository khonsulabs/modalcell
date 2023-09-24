(function() {var implementors = {
"modalcell":[["impl&lt;Mode&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.SharedMode.html\" title=\"struct modalcell::SharedMode\">SharedMode</a>&lt;Mode&gt;<span class=\"where fmt-newline\">where\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::SharedMode"]],["impl&lt;'a, Mode&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.ExclusiveMode.html\" title=\"struct modalcell::ExclusiveMode\">ExclusiveMode</a>&lt;'a, Mode&gt;<span class=\"where fmt-newline\">where\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::ExclusiveMode"]],["impl&lt;T, Mode&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.ExclusiveCell.html\" title=\"struct modalcell::ExclusiveCell\">ExclusiveCell</a>&lt;T, Mode&gt;<span class=\"where fmt-newline\">where\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/core/cell/struct.UnsafeCell.html\" title=\"struct core::cell::UnsafeCell\">UnsafeCell</a>&lt;T&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::ExclusiveCell"]],["impl&lt;T, Mode&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.SharedCell.html\" title=\"struct modalcell::SharedCell\">SharedCell</a>&lt;T, Mode&gt;<span class=\"where fmt-newline\">where\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/core/cell/struct.UnsafeCell.html\" title=\"struct core::cell::UnsafeCell\">UnsafeCell</a>&lt;T&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::SharedCell"]],["impl&lt;'a, 'mode, T, Mode&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.RefMut.html\" title=\"struct modalcell::RefMut\">RefMut</a>&lt;'a, 'mode, T, Mode&gt;<span class=\"where fmt-newline\">where\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.72.1/core/cell/struct.UnsafeCell.html\" title=\"struct core::cell::UnsafeCell\">UnsafeCell</a>&lt;T&gt;&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,\n    &lt;Mode as <a class=\"trait\" href=\"modalcell/trait.Mode.html\" title=\"trait modalcell::Mode\">Mode</a>&gt;::<a class=\"associatedtype\" href=\"modalcell/trait.Mode.html#associatedtype.Container\" title=\"type modalcell::Mode::Container\">Container</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.1/std/primitive.unit.html\">()</a>&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a>,</span>",1,["modalcell::RefMut"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.ThreadSafe.html\" title=\"struct modalcell::ThreadSafe\">ThreadSafe</a>",1,["modalcell::ThreadSafe"]],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.1/core/panic/unwind_safe/trait.RefUnwindSafe.html\" title=\"trait core::panic::unwind_safe::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"modalcell/struct.SingleThreaded.html\" title=\"struct modalcell::SingleThreaded\">SingleThreaded</a>",1,["modalcell::SingleThreaded"]]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()