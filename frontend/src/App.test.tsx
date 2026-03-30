import React from 'react';
import { render, screen } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import App from './App';

const renderWithRouter = (ui: React.ReactElement) => {
  return render(
    <BrowserRouter>{ui}</BrowserRouter>
  );
};

describe('Accessibility Audit Tool', () => {
  test('renders app without crashing', () => {
    renderWithRouter(<App />);
    const element = screen.getByText(/Accessibility Audit Tool/i);
    expect(element).toBeInTheDocument();
  });

  test('renders navigation links', () => {
    renderWithRouter(<App />);
    
    expect(screen.getByText(/Dashboard/i)).toBeInTheDocument();
    expect(screen.getByText(/Projects/i)).toBeInTheDocument();
    expect(screen.getByText(/New Audit/i)).toBeInTheDocument();
  });
});
