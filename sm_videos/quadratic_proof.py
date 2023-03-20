from manim import *
from pyglet.window import key as pyglet_key
#from manimlib import *
animations = []
animations2 = []
  
# mobject: MathTex 
def append_string_1(mobject):
	mobj_tex = mobject.get_tex_string()
	mobj_tex += "+(\\frac{b}{2})^2"
	return MathTex(mobj_tex).move_to(mobject)

class QuadraticProof(Scene):
	def construct(self):
		self.interactive_embed()
  
	def __init__(self, k):
		super().__init__()
		tex = Tex(r"Problem: $y=7x^2+11x+3$").move_to(UP * 3.5)
		tex2 = Tex(r"Completing the square: $(x+c)^2=d$").move_to(tex.get_center() + DOWN*1.4)
		animations.append(Write(tex, run_time=1.2))
		#self.play(Wait(run_time=0.5))
		#self.play(FadeOut(tex, run_time=0.3))
		animations.append(FadeIn(tex2))
		animations.append(FadeOut(tex2))

		tex3 = MathTex("x^2", "+", "\\frac{11}{7}x", "=", "-", "\\frac{3}{7}").move_to(tex2)
		tex4 = MathTex("x^2", "+", "\\frac{11}{7}x", "+", "(\\frac{b}{2})^2", "=", "-", "\\frac{3}{7}", "+", "(\\frac{b}{2})^2").move_to(tex2)
		tex5 = MathTex("x^2", "+", "\\frac{11}{7}x", "+", "\\frac{121}{196}", "=", "-", "\\frac{3}{7}", "+", "\\frac{121}{196}").move_to(tex2)
		tex6 = MathTex("x^2+\\frac{11}{7}x+\\frac{121}{196}=\\frac{37}{196}").move_to(tex2.get_center() + DOWN*1.4)
		tex7 = MathTex("(x+\\frac{11}{7})^2=\\frac{37}{196}").move_to(tex6.get_center() + DOWN*1.5)
		tex8 = MathTex("x=-\\frac{11}{7} \\pm \\sqrt{\\frac{37}{196}}").move_to(tex7.get_center() + DOWN*1.5)

		animations.append(FadeIn(tex3))
		animations.append(TransformMatchingTex(tex3, tex4))
		animations.append(TransformMatchingTex(tex4, tex5))
		animations.append(FadeIn(tex6))
		animations.append(FadeIn(tex7))
		animations.append(FadeIn(tex8))
  
	def on_key_press(self, symbol, modifiers):
		if symbol == pyglet_key.X:
			#self.play(animations[anim_index])
			self.play(animations.pop(0))
			#print(anim_index)
			#anim_index += 1
		super().on_key_press(symbol, modifiers)

class FormalProof(Scene):
	def construct(self):
		tex = MathTex("{{a}}x^2", "+", "{{b}}x", "+", "{{c}}").move_to(UP*3)
		tex2 = MathTex("{{1}}x^2", "+", "{{\\frac{11}{7}}}x", "+", "{{\\frac{3}{7}}}").move_to(tex)
		tex3 = MathTex("\\frac{x^2+bx}{a}", "=", "-\\frac{c}{a}").move_to(tex.get_center()+ DOWN*1.5)
		tex4 = MathTex("\\frac{x^2+bx}{a}", "+", "q", "=", "-\\frac{c}{a}", "+", "q").move_to(tex3.get_center()+ DOWN*1.5)
		tex5 = MathTex("q", "=", "?").move_to(tex4.get_center() + DOWN*7.5)
		self.add(tex)
		self.wait()
		self.play(TransformMatchingTex(tex, tex2))
		self.wait()
		self.play(TransformMatchingTex(tex2, tex))
		self.wait()
		self.play(FadeIn(tex3))
		self.wait()
		self.play(FadeIn(tex4))
		self.wait()
		
		tex3.generate_target()
		tex.generate_target()
		tex3.target.shift(UP*4.0)
		tex.target.shift(UP*4.0)
	
		# don't need mobj.animate.set_color_by_tex() because ApplyFunction already animates 
		def move_change_color(mobj):
			mobj.set_color_by_tex("q", YELLOW).shift(UP*2.5)
			return mobj

		def move_change_color2(mobj):
			mobj.set_color_by_tex("q", YELLOW).shift(UP*8.5)
			return mobj
		
		self.add(tex5)
		self.play(
			MoveToTarget(tex3),
			MoveToTarget(tex),
			ApplyFunction(move_change_color, tex4),
			ApplyFunction(move_change_color2, tex5),
		)
		self.wait()
  
		tex6 = Tex("Notice: $\\frac{121}{196}=(\\frac{11}{7} \div {2})^2$").move_to(tex5.get_center() + DOWN*1.5)
		tex7 = MathTex("q", "=", "(\\frac{b}{2})^2~?").move_to(tex5).set_color_by_tex("q", YELLOW)
		tex8 = MathTex("q", "=", "(\\frac{b}{2a})^2").move_to(tex5).set_color_by_tex("q", YELLOW)
		self.play(FadeIn(tex6))
		self.wait()
		self.play(TransformMatchingTex(tex5, tex7))
		self.wait()
		tex9 = MathTex("x^2", "+", "\\frac{b}{a}", "x", "+", "q", "=", "-\\frac{c}{a}", "+", "q").move_to(tex4)
		self.play(
			TransformMatchingTex(tex4, tex9),
			FadeOut(tex6)
		)
		self.wait()
		self.play(TransformMatchingTex(tex7, tex8))
		self.wait()
		self.play(FadeOut(tex8), FadeOut(tex9))
		self.wait()

class FormalProofPt2(Scene):
	def construct(self):
		tex = MathTex("(x+c)^2").shift(UP*3.0)
		tex2 = MathTex("(", "x", "+", "c", ")", "(", "x", "+", "c", ")").move_to(tex.get_center() + DOWN*1.5)
		# using VGroup.arrange_submobjects on strings results in weird spacing
		#tex2 = VGroup(MathTex("("), MathTex("x"), MathTex("+"), MathTex("c"), MathTex(")"), MathTex("("), MathTex("x"), MathTex("+"), MathTex("c"), MathTex(")")).arrange_submobjects().move_to(tex.get_center() + DOWN*1.5)
		x2 = tex2.get_parts_by_tex("x")
		y2 = tex2.get_parts_by_tex("c")
		tex4 = MathTex("x", "^2", "+", "2", "c", "x", "+", "c", "^2").move_to(tex2.get_center() + DOWN*1.5)
		#tex4 = VGroup(MathTex("x^2"), MathTex("+"), MathTex("2"), MathTex("x"), MathTex("c"), MathTex("y^2")).move_to(tex2.get_center() + DOWN*1.5).arrange_submobjects()
		self.play(FadeIn(tex))
		self.wait()
		self.play(FadeIn(tex2))
		x4 = tex4.get_parts_by_tex("x")
		y4 = tex4.get_parts_by_tex("c")
		
		# use .copy() instead of TransformFromCopy() which is kind of weird right now
		self.play(
			Transform(x2[0].copy(), x4[0]),
			Transform(x2[1].copy(), x4[0]),
			FadeIn(tex4[1])
		)
		self.play(
			Transform(x2[0].copy(), x4[1]),
			Transform(y2[1].copy(), y4[0]),
			FadeIn(tex4[2]),
		)
		self.play(
			Transform(x2[0].copy(), x4[1]),
			Transform(y2[1].copy(), y4[0]),
			#FadeIn(tex4[2]),
			FadeIn(tex4[3])
		)
		self.play(
			Transform(y2[0].copy(), y4[1]),
			Transform(y2[1].copy(), y4[1]),
			FadeIn(tex4[6]),
			FadeIn(tex4[8])
		)
  
		self.wait()


class FormalProofPt3(Scene):
	def construct(self):
		tex = MathTex("x^2", "+", "\\frac{b}{a}", "x", "+", "(\\frac{b}{2a})", "^2", "=", "(\\frac{b}{2a})", "^2", "-", "\\frac{c}{a}").shift(UP*3.2)
		self.add(tex)
		self.wait()
		a = VGroup(
			MathTex("\\frac{4a^2x^2+4abx+b^2}{4a^2}=\\frac{b^2-4ac}{4a^2}"),
			MathTex("4a^2x^2+4abx+b^2=b^2-4ac"),
			MathTex("(2ax+b)^2=b^2-4ac"),
			MathTex("2ax+b= \\pm \\sqrt{b^3-4ac}"),
		).arrange_in_grid(cols=1, buff=0.5).move_to(tex).shift(DOWN*3.0)
		for t in a:
			self.play(FadeIn(t))
			self.wait()
		final = MathTex("x= \\frac{b \\pm \\sqrt{b^2-4ac}}{2a}").move_to(a[len(a)-1].get_center()+ DOWN*1.35)
		self.play(FadeIn(final))	
		self.wait()
		self.play(
    		tex.animate.shift(UP*7.0),
			a.animate.shift(UP*7.0),
			final.animate.move_to(ORIGIN+ UP*2.0).set_color(YELLOW)
      	)
		self.wait()

