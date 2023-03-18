from manim import *


# mobject: MathTex 
def append_string_1(mobject):
	mobj_tex = mobject.get_tex_string()
	mobj_tex += "+(\\frac{b}{2})^2"
	return MathTex(mobj_tex).move_to(mobject)

class QuadraticProof(Scene):
	def construct(self):
		tex = Tex(r"Problem: $y=7x^2+11x+3$").move_to(UP * 3.5)
		tex2 = Tex(r"Completing the square: $(x+c)^2=d$").move_to(tex.get_center() + DOWN*1.4)

		self.play(Write(tex, run_time=1.2))
		#self.play(Wait(run_time=0.5))
		#self.play(FadeOut(tex, run_time=0.3))
		self.play(Wait(run_time=1.5))
		self.play(FadeIn(tex2))
		self.play(Wait(run_time=1.0))
		self.play(FadeOut(tex2))

		tex3 = MathTex("x^2", "+", "\\frac{11}{7}x", "=", "-", "\\frac{3}{7}").move_to(tex2)
		tex4 = MathTex("x^2", "+", "\\frac{11}{7}x", "+", "(\\frac{b}{2})^2", "=", "-", "\\frac{3}{7}", "+", "(\\frac{b}{2})^2").move_to(tex2)
		tex5 = MathTex("x^2", "+", "\\frac{11}{7}x", "+", "\\frac{121}{196}", "=", "-", "\\frac{3}{7}", "+", "\\frac{121}{196}").move_to(tex2)
		tex6 = MathTex("x^2+\\frac{11}{7}x+\\frac{121}{196}=\\frac{37}{196}").move_to(tex2.get_center() + DOWN*1.4)
		tex7 = MathTex("(x+\\frac{11}{7})^2=\\frac{37}{196}").move_to(tex6.get_center() + DOWN*1.5)
		tex8 = MathTex("x=-\\frac{11}{7} \\pm \\sqrt(\\frac{37}{196}").move_to(tex7.get_center() + DOWN*1.5)
		
		self.play(Wait(run_time=1.0))
		self.play(FadeIn(tex3))
		self.play(Wait(run_time=1.0))
		self.play(TransformMatchingTex(tex3, tex4))
		self.play(Wait(run_time=1.0))
		self.play(TransformMatchingTex(tex4, tex5))
		self.play(Wait(run_time=1.0))
		self.play(FadeIn(tex6))
		self.play(Wait(run_time=1.0))
		self.play(FadeIn(tex7))
		self.play(Wait(run_time=1.0))
		self.play(FadeIn(tex8))

class FormalProof(Scene):
	def construct(self):
		tex = MathTex("{{a}}x^2", "+", "{{b}}x", "+", "{{c}}").move_to(UP*3)
		tex2 = MathTex("{{1}}x^2", "+", "{{\\frac{11}{7}}}x", "+", "{{\\frac{3}{7}}}").move_to(tex)
		tex3 = MathTex("\\frac{x^2+bx}{a}", "=", "-\\frac{c}{a}").move_to(tex.get_center()+ DOWN*1.5)
		tex4 = MathTex("\\frac{x^2+bx}{a}", "+", "q", "=", "-\\frac{c}{a}", "+", "q").move_to(tex3.get_center()+ DOWN*1.5)
		tex5 = MathTex("q", "=", "?").move_to(tex4.get_center() + DOWN*1.5).set_color_by_tex("q", YELLOW)
		tex6 = Tex("Notice: $\\frac{121}{196}=(\\frac{11}{7} \div {2})^2$").move_to(tex5.get_center()+ DOWN*1.5)
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
		self.play(
			FadeIn(tex5.shift(UP*2.0)), 
	    	Transform(tex4, tex4.set_color_by_tex("q", YELLOW).shift(UP*2.0)),
		    Transform(tex, tex4.shift(UP*5.0)),
		    Transform(tex, tex3.shift(UP*5.0))
		)
		self.wait()
		self.play(FadeIn(tex6))
		self.wait()
		self.play(FadeOut(tex6), FadeOut(tex5))
		self.wait()

class FormalProof2(Scene):
	pass




