import * as React from 'react'
import TextareaAutosizeComponent from 'react-textarea-autosize'
import { cn } from '@/lib/utils'

const textareaStyles =
	'placeholder:text-white/50 ' +
	'focus-visible:border-primary focus-visible:ring-primary/50 ' +
	'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 ' +
	'aria-invalid:border-destructive flex field-sizing-content min-h-16 w-full rounded-lg border ' +
	'bg-black/20 border-white/20 px-3 py-2 text-base backdrop-blur-sm outline-none ' +
	'disabled:cursor-not-allowed disabled:opacity-50 md:text-sm';

function Textarea({ className, ...props }: React.ComponentProps<'textarea'>) {
	return (
		<textarea
			data-slot="textarea"
			className={cn(textareaStyles, className)}
			{...props}
		/>
	)
}

// TextareaAutosize component that uses the same styling as the Textarea component
function TextareaAutosize({
	className,
	...props
}: React.ComponentProps<typeof TextareaAutosizeComponent>) {
	return React.useMemo(() => (
		<TextareaAutosizeComponent
			data-slot="textarea"
			autoComplete='off'
			autoCorrect='off'
			className={cn(textareaStyles, className)}
			{...props}
		/>
	), [className, props]);
}

export { Textarea, TextareaAutosize }
