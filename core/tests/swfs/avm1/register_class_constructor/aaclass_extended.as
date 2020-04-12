class aaclass_extended extends aaclass {
	function aaclass_extended() {
		trace("aaclass_extended constructor");
		super();
		
		trace("");
		trace("// trace(this)");
		trace(this);
		trace("");
		trace("// trace(this._name)");
		trace(this._name);
		trace("");
		
		trace("aaclass_extended constructor end");
	}
}