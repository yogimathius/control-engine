import React from 'react';
import { cn } from '@/lib/utils';

export interface SelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
  variant?: 'default' | 'sacred' | 'mystic';
}

export interface SelectContentProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
}

export interface SelectItemProps extends React.OptionHTMLAttributes<HTMLOptionElement> {
  children?: React.ReactNode;
}

export interface SelectTriggerProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
}

export interface SelectValueProps extends React.HTMLAttributes<HTMLSpanElement> {
  placeholder?: string;
}

// For now, we'll create simplified versions that work with native select
const Select = React.forwardRef<HTMLSelectElement, SelectProps>(
  ({ className, variant = 'default', children, ...props }, ref) => (
    <select
      ref={ref}
      className={cn(
        'flex h-10 w-full rounded-md border border-slate-700 bg-slate-800/50 px-3 py-2 text-sm text-slate-100 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-slate-900 disabled:cursor-not-allowed disabled:opacity-50',
        className
      )}
      {...props}
    >
      {children}
    </select>
  )
);
Select.displayName = 'Select';

// These are placeholder components for compatibility
const SelectContent: React.FC<SelectContentProps> = ({ children }) => <>{children}</>;
const SelectItem: React.FC<SelectItemProps> = ({ children, value, ...props }) => (
  <option value={value} {...props}>{children}</option>
);
const SelectTrigger: React.FC<SelectTriggerProps> = ({ children }) => <>{children}</>;
const SelectValue: React.FC<SelectValueProps> = ({ placeholder }) => <>{placeholder}</>;

export { Select, SelectContent, SelectItem, SelectTrigger, SelectValue };