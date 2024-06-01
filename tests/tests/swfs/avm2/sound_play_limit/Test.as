package  {
	import flash.display.MovieClip;
	import flash.media.SoundChannel;
	import flash.utils.ByteArray;
	import flash.media.Sound;
	
	
	public class Test extends MovieClip {
		public function Test() {
			var result:Vector.<SoundChannel> = new Vector.<SoundChannel>();
			var soundChannel: SoundChannel;
			var soundCheck: Sound = new Silence();

			while ((soundChannel = soundCheck.play()) != null) {
				result.push(soundChannel);
			}

			trace("Available sound channels: " + result.length);
		}
	}	
}